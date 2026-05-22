from cryptography.hazmat.primitives.asymmetric import ec
from cryptography.hazmat.primitives import hashes, serialization
from cryptography.hazmat.primitives.asymmetric.utils import decode_dss_signature
import struct

sk = ec.generate_private_key(ec.SECP256R1())
vk = sk.public_key()
pubkey = vk.public_bytes(
    encoding=serialization.Encoding.X962,
    format=serialization.PublicFormat.UncompressedPoint
)

old_passkey = bytes([0]*65)
new_passkey = bytes([1]*65)
nonce_bytes = struct.pack(">I", 0) # u32 big endian

msg = old_passkey + new_passkey + nonce_bytes

signature = sk.sign(msg, ec.ECDSA(hashes.SHA256()))

r, s = decode_dss_signature(signature)
order = 0xFFFFFFFF00000000FFFFFFFFFFFFFFFFBCE6FAADA7179E84F3B9CAC2FC632551
if s > order // 2:
    s = order - s
sig = r.to_bytes(32, 'big') + s.to_bytes(32, 'big')

def to_hex_array(b):
    return "[" + ", ".join([hex(x) for x in b]) + "]"

print("pubkey:", to_hex_array(pubkey))
print("sig:", to_hex_array(sig))
