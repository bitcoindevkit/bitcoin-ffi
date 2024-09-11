package org.bitcoindevkit.bitcoinffi

import org.junit.jupiter.api.Test
import org.junit.jupiter.api.TestInstance

@TestInstance(TestInstance.Lifecycle.PER_CLASS)
class LibraryTest {

    @Test fun testNetwork() {
        val network0 = Network.BITCOIN
        val network1 = Network.REGTEST
        val network2 = Network.SIGNET
        val network3 = Network.TESTNET
    }
}
