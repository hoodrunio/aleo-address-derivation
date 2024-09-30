from aleo_address_derivation import signature_to_address

def get_address_from_signature(signature: str) -> str:
    try:
        return signature_to_address(signature)
    except ValueError as e:
        raise ValueError(f"Adres türetme hatası: {e}")