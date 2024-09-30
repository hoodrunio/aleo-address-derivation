from aleo_address_derivation import signature_to_address

def test_signature_to_address():
    test_signature = "sign1f8p5hs2g6069lqu2p3puv23hgx9r0cjfk7le43pv94tn4gdzhvpvqq345268xa2rwgzw0u0e3wkm2g6qu3p0sam5syalnfc489av7q5mqvetvqfwycxlnkaz4u77css6s2j9nuxz5w8g9uh6htgvl9ucq5ejn2pwnaxhslgefjz8q3pgwawee44epq76l4l453qqj4vqyresvc50zeg"
    
    try:
        address = signature_to_address(test_signature)
        print(f"Derived address: {address}")
    except ValueError as e:
        print(f"Error: {e}")

if __name__ == "__main__":
    test_signature_to_address()