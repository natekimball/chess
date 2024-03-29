import numpy as np
import tensorflow as tf
import sys

def fen_to_mat(fen):
    mat = np.zeros((13, 8, 8), dtype=np.int8)
    fen = fen.split(' ')
    i,j = 0,0
    for l in fen[0]:
        if l == ' ':
            break
        elif l == '/':
            i += 1
            j = 0
        elif l.isdigit():
            j += int(l)
        else:
            r = l.lower()
            if r == 'q':
                if l.isupper():
                    mat[0][i][j] = 1
                else:
                    mat[1][i][j] = 1
            elif r == 'k':
                if l.isupper():
                    mat[2][i][j] = 1
                else:
                    mat[3][i][j] = 1
            elif r == 'r':
                if l.isupper():
                    mat[4][i][j] = 1
                else:
                    mat[5][i][j] = 1
            elif r == 'b':
                if l.isupper():
                    mat[6][i][j] = 1
                else:
                    mat[7][i][j] = 1
            elif r == 'n':
                if l.isupper():
                    mat[8][i][j] = 1
                else:
                    mat[9][i][j] = 1
            elif r == 'p':
                if l.isupper():
                    mat[10][i][j] = 1
                else:
                    mat[11][i][j] = 1
            j += 1
    
    player = fen[1]
    castling_rights = fen[2]
    en_passant = fen[3]
    halfmove_clock = int(fen[4])
    fullmove_clock = int(fen[5])
    
    if en_passant != '-':
        mat[12, ord(en_passant[0]) - ord('a'), int(en_passant[1]) - 1] = 1
    if castling_rights != '-':
        for char in castling_rights:
            if char == 'K':
                mat[12, 7, 7] = 1
            elif char == 'k':
                mat[12, 0, 7] = 1
            elif char == 'Q':
                mat[12, 7, 0] = 1
            elif char == 'q':
                mat[12, 0, 0] = 1
    if player == 'w':
        mat[12, 7, 4] = 1
    else:
        mat[12, 0, 4] = 1
    if halfmove_clock > 0:
        c = 7
        while halfmove_clock > 0:
            mat[12, 3, c] = halfmove_clock%2
            halfmove_clock = halfmove_clock // 2
            c -= 1
            if c < 0:
                break
    if fullmove_clock > 0:
        c = 7
        while fullmove_clock > 0:
            mat[12, 4, c] = fullmove_clock%2
            fullmove_clock = fullmove_clock // 2
            c -= 1
            if c < 0:
                break
    return mat

def evaluation_to_int(evaluation):
    if evaluation.find('\ufeff') != -1:
        print("old",evaluation)
        evaluation = evaluation[1:]
        print("new",evaluation)
    if evaluation[0] == '#':
        evaluation = evaluation[1:]
    return int(evaluation)/10

def save_signatures(model, output_dir):
    optimizer = tf.keras.optimizers.Adam(learning_rate=1e-4)    
    custom_model = CustomModel(model, optimizer)

    tf.saved_model.save(
        custom_model,
        output_dir,
        signatures={
            'train': custom_model.train_step,
            'pred': custom_model.predict,
            'save': custom_model.save
        }
    )

train_input_signature = [
    tf.TensorSpec(shape=(None, 13, 8, 8), dtype=tf.float32, name='input'),
    tf.TensorSpec(shape=(None, 1), dtype=tf.float32, name='training_target')
]

pred_input_signature = [
    tf.TensorSpec(shape=(None, 13, 8, 8), dtype=tf.float32, name='input')
]

save_input_signature = [
    tf.TensorSpec(shape=(None, 1), dtype=tf.string, name='file_prefix')
]

class CustomModel(tf.keras.Model):
    def __init__(self, base_model, optimizer, **kwargs):
        super(CustomModel, self).__init__(**kwargs)
        self.base_model = base_model
        self.optimizer = optimizer
        self.checkpoint = tf.train.Checkpoint(model=self.base_model, optimizer=self.optimizer)

    @tf.function(input_signature=train_input_signature)
    def train_step(self, inputs, targets):
        with tf.GradientTape() as tape:
            predictions = self.base_model(inputs, training=True)
            loss = tf.keras.losses.mean_squared_error(targets, predictions)
        gradients = tape.gradient(loss, self.base_model.trainable_variables)
        self.optimizer.apply_gradients(zip(gradients, self.base_model.trainable_variables))
        return loss

    @tf.function(input_signature=pred_input_signature)
    def predict(self, inputs):
        return self.base_model(inputs, training=False)
    
    @tf.function
    def save(self):
        return self.checkpoint.write(file_prefix='model/training_checkpoints/ckpt')

def read_checkpoint(model):
    checkpoint = tf.train.Checkpoint(model=model)
    checkpoint.read(tf.train.latest_checkpoint('training_checkpoints')).assert_consumed()
    return model

def save_frozen(model):
    infer = model.signatures['serving_default']

    # Convert the SavedModel to a frozen graph
    frozen_func = tf.convert_variables_to_constants_v2(infer)
    frozen_func.graph.as_graph_def()

    # Save the frozen graph to a .pb file
    with tf.io.gfile.GFile('saved_model.pb', 'wb') as f:
        f.write(frozen_func.graph.as_graph_def().SerializeToString())

def get_arg(key, default):
    if key in sys.argv:
        return sys.argv[sys.argv.index(key) + 1]
    else:
        return default