import numpy as np
import sys
import onnx
import torch
from onnx2pytorch import ConvertModel
import tf2onnx

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
    return mat.tolist()

def evaluation_to_int(evaluation):
    if evaluation[0] == '#':
        return int(evaluation[1:])/10
    return int(evaluation)/10

def save_frozen(model):
    infer = loaded.signatures['serving_default']

    # Convert the SavedModel to a frozen graph
    frozen_func = convert_variables_to_constants_v2(infer)
    frozen_func.graph.as_graph_def()

    # Save the frozen graph to a .pb file
    with tf.io.gfile.GFile('saved_model.pb', 'wb') as f:
        f.write(frozen_func.graph.as_graph_def().SerializeToString())


def convert_tf_to_onnx(tf_model_path, onnx_model_path):
    # Load the TensorFlow model
    model_proto, _ = tf2onnx.convert.from_saved_model(tf_model_path)

    # Save the ONNX model
    with open(onnx_model_path, "wb") as f:
        f.write(model_proto.SerializeToString())

def convert_onnx_to_pytorch(onnx_model_path, pytorch_model_path):
    # Load the ONNX model
    onnx_model = onnx.load(onnx_model_path)

    # Convert the ONNX model to PyTorch
    pytorch_model = ConvertModel(onnx_model)

    # Save the PyTorch model
    torch.save(pytorch_model.state_dict(), pytorch_model_path)


def convert_tf_to_pytorch(tf_model_path, pytorch_model_path):
    convert_tf_to_onnx(tf_model_path, "onnx_model")
    convert_onnx_to_pytorch("onnx_model", pytorch_model_path)
    
if __name__ == "__main__":
    if len(sys.argv) != 3:
        print(f"Usage: {sys.argv[0]} <tensorflow_model_path> <pytorch_model_path>")
        sys.exit(1)

    tf_model_path = sys.argv[1]
    pytorch_model_path = sys.argv[2]

    convert_tf_to_pytorch(tf_model_path, pytorch_model_path)