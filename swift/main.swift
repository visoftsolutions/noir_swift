let BYTECODE = "H4sIAAAAAAAA/+1Y207DMAz1Mna/sCGQEDww9gV127TpG7/Sdt1n8yuwomTLSgYSPZmGwNKUas1O7ONL7D0S0RMdpKPXF70G7YTFDkM4cKMgieMqDSuOOA/CrFAyiGWRKFYsldyEKooqFas0K7I0yDiOKt7KLNpq4C5Oxw8sQZ8FzUUHqLND3bbYbB5sLq702tMfw4uw3r85OOtYz0LvEV/s6ZzAGVnfmd/Prfco4/ckGONqw14tY2vjV40zux7ONlg/TJBQJwhfES7ZeuQncAWYP6TNfQdWEZQbyUWySbnKpSrLLGIO8yRPilBtq0KykmqHWeah2h0X5iVXQZ5UXXIELOELTB+HFdj6DuhQCIQj/vsebKHGOU3+vBYBX84ZeMAdEi7ofdk9xPvIqSui0A094N6QnwKKuoCMnshYGgFjCMmfudBr/Z41h3ZRA8d/bPDBuKkgP8WWMHo6u8mxXif0x7rJ2nC7m6wJWDWUA50Z1/hdwhexW7rsIqblKDHadtFjIH93Z+KvrZ4TYPwDY4aB/B0VPPQUgoyZKf2+KWRK2MvayIz+pxCIc2YecOd02VPIVDsL7CMvU8ikEVgo3Hu67Avc6ImMpWtgDCH5M41grZ9rCjEiwDEA/CuPAY3CfkpwmO5lAlnodUmnJ5C1tf/UBLKm7ycQF87ZJxD7cBTmQhOJLlALwidY7ehVg1t0x+wrqdrq9UDnKfhtp6wF0OYl0BcA/pwFzkc+ovg7VfCa8g4KyrarOR4AAA=="

func testProveVerify() -> Bool {
    print("Initializing witness...");
    let witness = RustVec<Int32>()
    witness.push(value: 47)
    witness.push(value: 2)
    print("Generating proof...");
    guard let proof = prove_swift(BYTECODE, witness) else {
        return false
    }
    print("Verifying proof...");
    return verify_swift(BYTECODE, proof) ?? false
}

assert(testProveVerify())
print("Proof correct");