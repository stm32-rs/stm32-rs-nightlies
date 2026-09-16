#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    sr: SR,
    din: DIN,
    dout: DOUT,
    dmacr: DMACR,
    imscr: IMSCR,
    risr: RISR,
    misr: MISR,
    k0lr: K0LR,
    k0rr: K0RR,
    k1lr: K1LR,
    k1rr: K1RR,
    k2lr: K2LR,
    k2rr: K2RR,
    k3lr: K3LR,
    k3rr: K3RR,
    iv0lr: IV0LR,
    iv0rr: IV0RR,
    iv1lr: IV1LR,
    iv1rr: IV1RR,
    csgcmccm0r: CSGCMCCM0R,
    csgcmccm1r: CSGCMCCM1R,
    csgcmccm2r: CSGCMCCM2R,
    csgcmccm3r: CSGCMCCM3R,
    csgcmccm4r: CSGCMCCM4R,
    csgcmccm5r: CSGCMCCM5R,
    csgcmccm6r: CSGCMCCM6R,
    csgcmccm7r: CSGCMCCM7R,
    csgcm0r: CSGCM0R,
    csgcm1r: CSGCM1R,
    csgcm2r: CSGCM2R,
    csgcm3r: CSGCM3R,
    csgcm4r: CSGCM4R,
    csgcm5r: CSGCM5R,
    csgcm6r: CSGCM6R,
    csgcm7r: CSGCM7R,
    _reserved36: [u8; 0x0360],
    hwcfgr: HWCFGR,
    verr: VERR,
    ipidr: IPIDR,
    mid: MID,
}
impl RegisterBlock {
    ///0x00 - CRYP control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x04 - CRYP status register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x08 - The CRYP_DIN register is the data input register. It is 32-bit wide. It is used to enter into the input FIFO up to four 64-bit blocks (TDES) or two 128-bit blocks (AES) of plaintext (when encrypting) or ciphertext (when decrypting), one 32-bit word at a time. To fit different data sizes, the data can be swapped after processing by configuring the DATATYPE bits in the CRYP_CR register. Refer to Section39.3.16: CRYP data registers and data swapping for more details. When CRYP_DIN register is written to the data are pushed into the input FIFO. If CRYPEN = 1, when at least two 32-bit words in the DES/TDES mode have been pushed into the input FIFO (four words in the AES mode), and when at least two words are free in the output FIFO (four words in the AES mode), the CRYP engine starts an encrypting or decrypting process. When CRYP_DIN register is read: If CRYPEN = 0, the FIFO is popped, and then the data present in the Input FIFO are returned, from the oldest one (first reading) to the newest one (last reading). The IFEM flag must be checked before each read operation to make sure that the FIFO is not empty. if CRYPEN = 1, an undefined value is returned. After the CRYP_DIN register has been read once or several times, the FIFO must be flushed by setting the FFLUSH bit prior to processing new data.
    #[inline(always)]
    pub const fn din(&self) -> &DIN {
        &self.din
    }
    ///0x0c - The CRYP_DOUT register is the data output register. It is read-only and 32-bit wide. It is used to retrieve from the output FIFO up to four 64-bit blocks (TDES) or two 128-bit blocks (AES) of plaintext (when encrypting) or ciphertext (when decrypting), one 32-bit word at a time. To fit different data sizes, the data can be swapped after processing by configuring the DATATYPE bits in the CRYP_CR register. Refer to Section39.3.16: CRYP data registers and data swapping for more details. When CRYP_DOUT register is read, the last data entered into the output FIFO (pointed to by the read pointer) is returned.
    #[inline(always)]
    pub const fn dout(&self) -> &DOUT {
        &self.dout
    }
    ///0x10 - CRYP DMA control register
    #[inline(always)]
    pub const fn dmacr(&self) -> &DMACR {
        &self.dmacr
    }
    ///0x14 - The CRYP_IMSCR register is the interrupt mask set or clear register. It is a read/write register. When a read operation is performed, this register gives the current value of the mask applied to the relevant interrupt. Writing 1 to the particular bit sets the mask, thus enabling the interrupt to be read. Writing 0 to this bit clears the corresponding mask. All the bits are cleared to 0 when the peripheral is reset.
    #[inline(always)]
    pub const fn imscr(&self) -> &IMSCR {
        &self.imscr
    }
    ///0x18 - The CRYP_RISR register is the raw interrupt status register. It is a read-only register. When a read operation is performed, this register gives the current raw status of the corresponding interrupt, i.e. the interrupt information without taking CRYP_IMSCR mask into account. Write operations have no effect.
    #[inline(always)]
    pub const fn risr(&self) -> &RISR {
        &self.risr
    }
    ///0x1c - The CRYP_MISR register is the masked interrupt status register. It is a read-only register. When a read operation is performed, this register gives the current masked status of the corresponding interrupt, i.e. the interrupt information taking CRYP_IMSCR mask into account. Write operations have no effect.
    #[inline(always)]
    pub const fn misr(&self) -> &MISR {
        &self.misr
    }
    ///0x20 - CRYP key registers contain the cryptographic keys. In DES/TDES mode, the keys are 64-bit binary values (number from left to right, that is the leftmost bit is bit 1) and named K1, K2 and K3 (K0 is not used). Each key consists of 56 information bits and 8 parity bits. In AES mode, the key is considered as a single 128, 192 or 256 bits long sequence K0K1K2...K127/191/255. The AES key is entered into the registers as follows: for AES-128: K0..K127 corresponds to b127..b0 (b255..b128 are not used), for AES-192: K0..K191 corresponds to b191..b0 (b255..b192 are not used), for AES-256: K0..K255 corresponds to b255..b0. In all cases key bit K0 is the leftmost bit in CRYP inner memory and register bit b0 is the rightmost bit in corresponding CRYP_KxLR key register. For more information refer to Section39.3.17: CRYP key registers. Write accesses to these registers are disregarded when the cryptographic processor is busy (bit BUSY = 1 in the CRYP_SR register)
    #[inline(always)]
    pub const fn k0lr(&self) -> &K0LR {
        &self.k0lr
    }
    ///0x24 - Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details.
    #[inline(always)]
    pub const fn k0rr(&self) -> &K0RR {
        &self.k0rr
    }
    ///0x28 - Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details.
    #[inline(always)]
    pub const fn k1lr(&self) -> &K1LR {
        &self.k1lr
    }
    ///0x2c - Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details.
    #[inline(always)]
    pub const fn k1rr(&self) -> &K1RR {
        &self.k1rr
    }
    ///0x30 - Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details.
    #[inline(always)]
    pub const fn k2lr(&self) -> &K2LR {
        &self.k2lr
    }
    ///0x34 - Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details.
    #[inline(always)]
    pub const fn k2rr(&self) -> &K2RR {
        &self.k2rr
    }
    ///0x38 - Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details.
    #[inline(always)]
    pub const fn k3lr(&self) -> &K3LR {
        &self.k3lr
    }
    ///0x3c - Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details.
    #[inline(always)]
    pub const fn k3rr(&self) -> &K3RR {
        &self.k3rr
    }
    ///0x40 - The CRYP_IV0...1(L/R)R are the left-word and right-word registers for the initialization vector (64 bits for DES/TDES and 128 bits for AES). For more information refer to Section39.3.18: CRYP initialization vector registers. IV0 is the leftmost bit whereas IV63 (DES, TDES) or IV127 (AES) are the rightmost bits of the initialization vector. IV1(L/R)R is used only in the AES. Only CRYP_IV0(L/R) is used in DES/TDES. Write access to these registers are disregarded when the cryptographic processor is busy (bit BUSY = 1 in the CRYP_SR register).
    #[inline(always)]
    pub const fn iv0lr(&self) -> &IV0LR {
        &self.iv0lr
    }
    ///0x44 - Refer to Section39.6.17: CRYP initialization vector register 0L (CRYP_IV0LR) for details.
    #[inline(always)]
    pub const fn iv0rr(&self) -> &IV0RR {
        &self.iv0rr
    }
    ///0x48 - Refer to Section39.6.17: CRYP initialization vector register 0L (CRYP_IV0LR) for details.
    #[inline(always)]
    pub const fn iv1lr(&self) -> &IV1LR {
        &self.iv1lr
    }
    ///0x4c - Refer to Section39.6.17: CRYP initialization vector register 0L (CRYP_IV0LR) for details.
    #[inline(always)]
    pub const fn iv1rr(&self) -> &IV1RR {
        &self.iv1rr
    }
    ///0x50 - These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers.
    #[inline(always)]
    pub const fn csgcmccm0r(&self) -> &CSGCMCCM0R {
        &self.csgcmccm0r
    }
    ///0x54 - These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers.
    #[inline(always)]
    pub const fn csgcmccm1r(&self) -> &CSGCMCCM1R {
        &self.csgcmccm1r
    }
    ///0x58 - These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers.
    #[inline(always)]
    pub const fn csgcmccm2r(&self) -> &CSGCMCCM2R {
        &self.csgcmccm2r
    }
    ///0x5c - These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers.
    #[inline(always)]
    pub const fn csgcmccm3r(&self) -> &CSGCMCCM3R {
        &self.csgcmccm3r
    }
    ///0x60 - These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers.
    #[inline(always)]
    pub const fn csgcmccm4r(&self) -> &CSGCMCCM4R {
        &self.csgcmccm4r
    }
    ///0x64 - These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers.
    #[inline(always)]
    pub const fn csgcmccm5r(&self) -> &CSGCMCCM5R {
        &self.csgcmccm5r
    }
    ///0x68 - These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers.
    #[inline(always)]
    pub const fn csgcmccm6r(&self) -> &CSGCMCCM6R {
        &self.csgcmccm6r
    }
    ///0x6c - These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers.
    #[inline(always)]
    pub const fn csgcmccm7r(&self) -> &CSGCMCCM7R {
        &self.csgcmccm7r
    }
    ///0x70 - Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details.
    #[inline(always)]
    pub const fn csgcm0r(&self) -> &CSGCM0R {
        &self.csgcm0r
    }
    ///0x74 - Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details.
    #[inline(always)]
    pub const fn csgcm1r(&self) -> &CSGCM1R {
        &self.csgcm1r
    }
    ///0x78 - Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details.
    #[inline(always)]
    pub const fn csgcm2r(&self) -> &CSGCM2R {
        &self.csgcm2r
    }
    ///0x7c - Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details.
    #[inline(always)]
    pub const fn csgcm3r(&self) -> &CSGCM3R {
        &self.csgcm3r
    }
    ///0x80 - Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details.
    #[inline(always)]
    pub const fn csgcm4r(&self) -> &CSGCM4R {
        &self.csgcm4r
    }
    ///0x84 - Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details.
    #[inline(always)]
    pub const fn csgcm5r(&self) -> &CSGCM5R {
        &self.csgcm5r
    }
    ///0x88 - Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details.
    #[inline(always)]
    pub const fn csgcm6r(&self) -> &CSGCM6R {
        &self.csgcm6r
    }
    ///0x8c - Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details.
    #[inline(always)]
    pub const fn csgcm7r(&self) -> &CSGCM7R {
        &self.csgcm7r
    }
    ///0x3f0 - CRYP hardware configuration register
    #[inline(always)]
    pub const fn hwcfgr(&self) -> &HWCFGR {
        &self.hwcfgr
    }
    ///0x3f4 - CRYP HW Version Register
    #[inline(always)]
    pub const fn verr(&self) -> &VERR {
        &self.verr
    }
    ///0x3f8 - CRYP Identification
    #[inline(always)]
    pub const fn ipidr(&self) -> &IPIDR {
        &self.ipidr
    }
    ///0x3fc - CRYP HW Magic ID
    #[inline(always)]
    pub const fn mid(&self) -> &MID {
        &self.mid
    }
}
/**CR (rw) register accessor: CRYP control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#CRYP1:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///CRYP control register
pub mod cr;
/**SR (r) register accessor: CRYP status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#CRYP1:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///CRYP status register
pub mod sr;
/**DIN (rw) register accessor: The CRYP_DIN register is the data input register. It is 32-bit wide. It is used to enter into the input FIFO up to four 64-bit blocks (TDES) or two 128-bit blocks (AES) of plaintext (when encrypting) or ciphertext (when decrypting), one 32-bit word at a time. To fit different data sizes, the data can be swapped after processing by configuring the DATATYPE bits in the CRYP_CR register. Refer to Section39.3.16: CRYP data registers and data swapping for more details. When CRYP_DIN register is written to the data are pushed into the input FIFO. If CRYPEN = 1, when at least two 32-bit words in the DES/TDES mode have been pushed into the input FIFO (four words in the AES mode), and when at least two words are free in the output FIFO (four words in the AES mode), the CRYP engine starts an encrypting or decrypting process. When CRYP_DIN register is read: If CRYPEN = 0, the FIFO is popped, and then the data present in the Input FIFO are returned, from the oldest one (first reading) to the newest one (last reading). The IFEM flag must be checked before each read operation to make sure that the FIFO is not empty. if CRYPEN = 1, an undefined value is returned. After the CRYP_DIN register has been read once or several times, the FIFO must be flushed by setting the FFLUSH bit prior to processing new data.

You can [`read`](crate::Reg::read) this register and get [`din::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`din::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#CRYP1:DIN)

For information about available fields see [`mod@din`] module*/
pub type DIN = crate::Reg<din::DINrs>;
///The CRYP_DIN register is the data input register. It is 32-bit wide. It is used to enter into the input FIFO up to four 64-bit blocks (TDES) or two 128-bit blocks (AES) of plaintext (when encrypting) or ciphertext (when decrypting), one 32-bit word at a time. To fit different data sizes, the data can be swapped after processing by configuring the DATATYPE bits in the CRYP_CR register. Refer to Section39.3.16: CRYP data registers and data swapping for more details. When CRYP_DIN register is written to the data are pushed into the input FIFO. If CRYPEN = 1, when at least two 32-bit words in the DES/TDES mode have been pushed into the input FIFO (four words in the AES mode), and when at least two words are free in the output FIFO (four words in the AES mode), the CRYP engine starts an encrypting or decrypting process. When CRYP_DIN register is read: If CRYPEN = 0, the FIFO is popped, and then the data present in the Input FIFO are returned, from the oldest one (first reading) to the newest one (last reading). The IFEM flag must be checked before each read operation to make sure that the FIFO is not empty. if CRYPEN = 1, an undefined value is returned. After the CRYP_DIN register has been read once or several times, the FIFO must be flushed by setting the FFLUSH bit prior to processing new data.
pub mod din;
/**DOUT (r) register accessor: The CRYP_DOUT register is the data output register. It is read-only and 32-bit wide. It is used to retrieve from the output FIFO up to four 64-bit blocks (TDES) or two 128-bit blocks (AES) of plaintext (when encrypting) or ciphertext (when decrypting), one 32-bit word at a time. To fit different data sizes, the data can be swapped after processing by configuring the DATATYPE bits in the CRYP_CR register. Refer to Section39.3.16: CRYP data registers and data swapping for more details. When CRYP_DOUT register is read, the last data entered into the output FIFO (pointed to by the read pointer) is returned.

You can [`read`](crate::Reg::read) this register and get [`dout::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#CRYP1:DOUT)

For information about available fields see [`mod@dout`] module*/
pub type DOUT = crate::Reg<dout::DOUTrs>;
///The CRYP_DOUT register is the data output register. It is read-only and 32-bit wide. It is used to retrieve from the output FIFO up to four 64-bit blocks (TDES) or two 128-bit blocks (AES) of plaintext (when encrypting) or ciphertext (when decrypting), one 32-bit word at a time. To fit different data sizes, the data can be swapped after processing by configuring the DATATYPE bits in the CRYP_CR register. Refer to Section39.3.16: CRYP data registers and data swapping for more details. When CRYP_DOUT register is read, the last data entered into the output FIFO (pointed to by the read pointer) is returned.
pub mod dout;
/**DMACR (rw) register accessor: CRYP DMA control register

You can [`read`](crate::Reg::read) this register and get [`dmacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#CRYP1:DMACR)

For information about available fields see [`mod@dmacr`] module*/
pub type DMACR = crate::Reg<dmacr::DMACRrs>;
///CRYP DMA control register
pub mod dmacr;
/**IMSCR (rw) register accessor: The CRYP_IMSCR register is the interrupt mask set or clear register. It is a read/write register. When a read operation is performed, this register gives the current value of the mask applied to the relevant interrupt. Writing 1 to the particular bit sets the mask, thus enabling the interrupt to be read. Writing 0 to this bit clears the corresponding mask. All the bits are cleared to 0 when the peripheral is reset.

You can [`read`](crate::Reg::read) this register and get [`imscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#CRYP1:IMSCR)

For information about available fields see [`mod@imscr`] module*/
pub type IMSCR = crate::Reg<imscr::IMSCRrs>;
///The CRYP_IMSCR register is the interrupt mask set or clear register. It is a read/write register. When a read operation is performed, this register gives the current value of the mask applied to the relevant interrupt. Writing 1 to the particular bit sets the mask, thus enabling the interrupt to be read. Writing 0 to this bit clears the corresponding mask. All the bits are cleared to 0 when the peripheral is reset.
pub mod imscr;
/**RISR (r) register accessor: The CRYP_RISR register is the raw interrupt status register. It is a read-only register. When a read operation is performed, this register gives the current raw status of the corresponding interrupt, i.e. the interrupt information without taking CRYP_IMSCR mask into account. Write operations have no effect.

You can [`read`](crate::Reg::read) this register and get [`risr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#CRYP1:RISR)

For information about available fields see [`mod@risr`] module*/
pub type RISR = crate::Reg<risr::RISRrs>;
///The CRYP_RISR register is the raw interrupt status register. It is a read-only register. When a read operation is performed, this register gives the current raw status of the corresponding interrupt, i.e. the interrupt information without taking CRYP_IMSCR mask into account. Write operations have no effect.
pub mod risr;
/**MISR (r) register accessor: The CRYP_MISR register is the masked interrupt status register. It is a read-only register. When a read operation is performed, this register gives the current masked status of the corresponding interrupt, i.e. the interrupt information taking CRYP_IMSCR mask into account. Write operations have no effect.

You can [`read`](crate::Reg::read) this register and get [`misr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#CRYP1:MISR)

For information about available fields see [`mod@misr`] module*/
pub type MISR = crate::Reg<misr::MISRrs>;
///The CRYP_MISR register is the masked interrupt status register. It is a read-only register. When a read operation is performed, this register gives the current masked status of the corresponding interrupt, i.e. the interrupt information taking CRYP_IMSCR mask into account. Write operations have no effect.
pub mod misr;
/**K0LR (w) register accessor: CRYP key registers contain the cryptographic keys. In DES/TDES mode, the keys are 64-bit binary values (number from left to right, that is the leftmost bit is bit 1) and named K1, K2 and K3 (K0 is not used). Each key consists of 56 information bits and 8 parity bits. In AES mode, the key is considered as a single 128, 192 or 256 bits long sequence K0K1K2...K127/191/255. The AES key is entered into the registers as follows: for AES-128: K0..K127 corresponds to b127..b0 (b255..b128 are not used), for AES-192: K0..K191 corresponds to b191..b0 (b255..b192 are not used), for AES-256: K0..K255 corresponds to b255..b0. In all cases key bit K0 is the leftmost bit in CRYP inner memory and register bit b0 is the rightmost bit in corresponding CRYP_KxLR key register. For more information refer to Section39.3.17: CRYP key registers. Write accesses to these registers are disregarded when the cryptographic processor is busy (bit BUSY = 1 in the CRYP_SR register)

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`k0lr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#CRYP1:K0LR)

For information about available fields see [`mod@k0lr`] module*/
pub type K0LR = crate::Reg<k0lr::K0LRrs>;
///CRYP key registers contain the cryptographic keys. In DES/TDES mode, the keys are 64-bit binary values (number from left to right, that is the leftmost bit is bit 1) and named K1, K2 and K3 (K0 is not used). Each key consists of 56 information bits and 8 parity bits. In AES mode, the key is considered as a single 128, 192 or 256 bits long sequence K0K1K2...K127/191/255. The AES key is entered into the registers as follows: for AES-128: K0..K127 corresponds to b127..b0 (b255..b128 are not used), for AES-192: K0..K191 corresponds to b191..b0 (b255..b192 are not used), for AES-256: K0..K255 corresponds to b255..b0. In all cases key bit K0 is the leftmost bit in CRYP inner memory and register bit b0 is the rightmost bit in corresponding CRYP_KxLR key register. For more information refer to Section39.3.17: CRYP key registers. Write accesses to these registers are disregarded when the cryptographic processor is busy (bit BUSY = 1 in the CRYP_SR register)
pub mod k0lr;
/**K0RR (w) register accessor: Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details.

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`k0rr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#CRYP1:K0RR)

For information about available fields see [`mod@k0rr`] module*/
pub type K0RR = crate::Reg<k0rr::K0RRrs>;
///Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details.
pub mod k0rr;
/**K1LR (w) register accessor: Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details.

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`k1lr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#CRYP1:K1LR)

For information about available fields see [`mod@k1lr`] module*/
pub type K1LR = crate::Reg<k1lr::K1LRrs>;
///Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details.
pub mod k1lr;
/**K1RR (w) register accessor: Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details.

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`k1rr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#CRYP1:K1RR)

For information about available fields see [`mod@k1rr`] module*/
pub type K1RR = crate::Reg<k1rr::K1RRrs>;
///Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details.
pub mod k1rr;
/**K2LR (w) register accessor: Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details.

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`k2lr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#CRYP1:K2LR)

For information about available fields see [`mod@k2lr`] module*/
pub type K2LR = crate::Reg<k2lr::K2LRrs>;
///Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details.
pub mod k2lr;
/**K2RR (w) register accessor: Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details.

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`k2rr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#CRYP1:K2RR)

For information about available fields see [`mod@k2rr`] module*/
pub type K2RR = crate::Reg<k2rr::K2RRrs>;
///Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details.
pub mod k2rr;
/**K3LR (w) register accessor: Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details.

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`k3lr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#CRYP1:K3LR)

For information about available fields see [`mod@k3lr`] module*/
pub type K3LR = crate::Reg<k3lr::K3LRrs>;
///Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details.
pub mod k3lr;
/**K3RR (w) register accessor: Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details.

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`k3rr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#CRYP1:K3RR)

For information about available fields see [`mod@k3rr`] module*/
pub type K3RR = crate::Reg<k3rr::K3RRrs>;
///Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details.
pub mod k3rr;
/**IV0LR (rw) register accessor: The CRYP_IV0...1(L/R)R are the left-word and right-word registers for the initialization vector (64 bits for DES/TDES and 128 bits for AES). For more information refer to Section39.3.18: CRYP initialization vector registers. IV0 is the leftmost bit whereas IV63 (DES, TDES) or IV127 (AES) are the rightmost bits of the initialization vector. IV1(L/R)R is used only in the AES. Only CRYP_IV0(L/R) is used in DES/TDES. Write access to these registers are disregarded when the cryptographic processor is busy (bit BUSY = 1 in the CRYP_SR register).

You can [`read`](crate::Reg::read) this register and get [`iv0lr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iv0lr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#CRYP1:IV0LR)

For information about available fields see [`mod@iv0lr`] module*/
pub type IV0LR = crate::Reg<iv0lr::IV0LRrs>;
///The CRYP_IV0...1(L/R)R are the left-word and right-word registers for the initialization vector (64 bits for DES/TDES and 128 bits for AES). For more information refer to Section39.3.18: CRYP initialization vector registers. IV0 is the leftmost bit whereas IV63 (DES, TDES) or IV127 (AES) are the rightmost bits of the initialization vector. IV1(L/R)R is used only in the AES. Only CRYP_IV0(L/R) is used in DES/TDES. Write access to these registers are disregarded when the cryptographic processor is busy (bit BUSY = 1 in the CRYP_SR register).
pub mod iv0lr;
/**IV0RR (rw) register accessor: Refer to Section39.6.17: CRYP initialization vector register 0L (CRYP_IV0LR) for details.

You can [`read`](crate::Reg::read) this register and get [`iv0rr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iv0rr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#CRYP1:IV0RR)

For information about available fields see [`mod@iv0rr`] module*/
pub type IV0RR = crate::Reg<iv0rr::IV0RRrs>;
///Refer to Section39.6.17: CRYP initialization vector register 0L (CRYP_IV0LR) for details.
pub mod iv0rr;
/**IV1LR (rw) register accessor: Refer to Section39.6.17: CRYP initialization vector register 0L (CRYP_IV0LR) for details.

You can [`read`](crate::Reg::read) this register and get [`iv1lr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iv1lr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#CRYP1:IV1LR)

For information about available fields see [`mod@iv1lr`] module*/
pub type IV1LR = crate::Reg<iv1lr::IV1LRrs>;
///Refer to Section39.6.17: CRYP initialization vector register 0L (CRYP_IV0LR) for details.
pub mod iv1lr;
/**IV1RR (rw) register accessor: Refer to Section39.6.17: CRYP initialization vector register 0L (CRYP_IV0LR) for details.

You can [`read`](crate::Reg::read) this register and get [`iv1rr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iv1rr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#CRYP1:IV1RR)

For information about available fields see [`mod@iv1rr`] module*/
pub type IV1RR = crate::Reg<iv1rr::IV1RRrs>;
///Refer to Section39.6.17: CRYP initialization vector register 0L (CRYP_IV0LR) for details.
pub mod iv1rr;
/**CSGCMCCM0R (rw) register accessor: These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers.

You can [`read`](crate::Reg::read) this register and get [`csgcmccm0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcmccm0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#CRYP1:CSGCMCCM0R)

For information about available fields see [`mod@csgcmccm0r`] module*/
pub type CSGCMCCM0R = crate::Reg<csgcmccm0r::CSGCMCCM0Rrs>;
///These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers.
pub mod csgcmccm0r;
/**CSGCMCCM1R (rw) register accessor: These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers.

You can [`read`](crate::Reg::read) this register and get [`csgcmccm1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcmccm1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#CRYP1:CSGCMCCM1R)

For information about available fields see [`mod@csgcmccm1r`] module*/
pub type CSGCMCCM1R = crate::Reg<csgcmccm1r::CSGCMCCM1Rrs>;
///These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers.
pub mod csgcmccm1r;
/**CSGCMCCM2R (rw) register accessor: These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers.

You can [`read`](crate::Reg::read) this register and get [`csgcmccm2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcmccm2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#CRYP1:CSGCMCCM2R)

For information about available fields see [`mod@csgcmccm2r`] module*/
pub type CSGCMCCM2R = crate::Reg<csgcmccm2r::CSGCMCCM2Rrs>;
///These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers.
pub mod csgcmccm2r;
/**CSGCMCCM3R (rw) register accessor: These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers.

You can [`read`](crate::Reg::read) this register and get [`csgcmccm3r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcmccm3r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#CRYP1:CSGCMCCM3R)

For information about available fields see [`mod@csgcmccm3r`] module*/
pub type CSGCMCCM3R = crate::Reg<csgcmccm3r::CSGCMCCM3Rrs>;
///These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers.
pub mod csgcmccm3r;
/**CSGCMCCM4R (rw) register accessor: These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers.

You can [`read`](crate::Reg::read) this register and get [`csgcmccm4r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcmccm4r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#CRYP1:CSGCMCCM4R)

For information about available fields see [`mod@csgcmccm4r`] module*/
pub type CSGCMCCM4R = crate::Reg<csgcmccm4r::CSGCMCCM4Rrs>;
///These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers.
pub mod csgcmccm4r;
/**CSGCMCCM5R (rw) register accessor: These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers.

You can [`read`](crate::Reg::read) this register and get [`csgcmccm5r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcmccm5r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#CRYP1:CSGCMCCM5R)

For information about available fields see [`mod@csgcmccm5r`] module*/
pub type CSGCMCCM5R = crate::Reg<csgcmccm5r::CSGCMCCM5Rrs>;
///These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers.
pub mod csgcmccm5r;
/**CSGCMCCM6R (rw) register accessor: These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers.

You can [`read`](crate::Reg::read) this register and get [`csgcmccm6r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcmccm6r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#CRYP1:CSGCMCCM6R)

For information about available fields see [`mod@csgcmccm6r`] module*/
pub type CSGCMCCM6R = crate::Reg<csgcmccm6r::CSGCMCCM6Rrs>;
///These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers.
pub mod csgcmccm6r;
/**CSGCMCCM7R (rw) register accessor: These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers.

You can [`read`](crate::Reg::read) this register and get [`csgcmccm7r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcmccm7r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#CRYP1:CSGCMCCM7R)

For information about available fields see [`mod@csgcmccm7r`] module*/
pub type CSGCMCCM7R = crate::Reg<csgcmccm7r::CSGCMCCM7Rrs>;
///These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers.
pub mod csgcmccm7r;
/**CSGCM0R (rw) register accessor: Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details.

You can [`read`](crate::Reg::read) this register and get [`csgcm0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcm0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#CRYP1:CSGCM0R)

For information about available fields see [`mod@csgcm0r`] module*/
pub type CSGCM0R = crate::Reg<csgcm0r::CSGCM0Rrs>;
///Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details.
pub mod csgcm0r;
/**CSGCM1R (rw) register accessor: Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details.

You can [`read`](crate::Reg::read) this register and get [`csgcm1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcm1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#CRYP1:CSGCM1R)

For information about available fields see [`mod@csgcm1r`] module*/
pub type CSGCM1R = crate::Reg<csgcm1r::CSGCM1Rrs>;
///Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details.
pub mod csgcm1r;
/**CSGCM2R (rw) register accessor: Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details.

You can [`read`](crate::Reg::read) this register and get [`csgcm2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcm2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#CRYP1:CSGCM2R)

For information about available fields see [`mod@csgcm2r`] module*/
pub type CSGCM2R = crate::Reg<csgcm2r::CSGCM2Rrs>;
///Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details.
pub mod csgcm2r;
/**CSGCM3R (rw) register accessor: Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details.

You can [`read`](crate::Reg::read) this register and get [`csgcm3r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcm3r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#CRYP1:CSGCM3R)

For information about available fields see [`mod@csgcm3r`] module*/
pub type CSGCM3R = crate::Reg<csgcm3r::CSGCM3Rrs>;
///Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details.
pub mod csgcm3r;
/**CSGCM4R (rw) register accessor: Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details.

You can [`read`](crate::Reg::read) this register and get [`csgcm4r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcm4r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#CRYP1:CSGCM4R)

For information about available fields see [`mod@csgcm4r`] module*/
pub type CSGCM4R = crate::Reg<csgcm4r::CSGCM4Rrs>;
///Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details.
pub mod csgcm4r;
/**CSGCM5R (rw) register accessor: Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details.

You can [`read`](crate::Reg::read) this register and get [`csgcm5r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcm5r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#CRYP1:CSGCM5R)

For information about available fields see [`mod@csgcm5r`] module*/
pub type CSGCM5R = crate::Reg<csgcm5r::CSGCM5Rrs>;
///Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details.
pub mod csgcm5r;
/**CSGCM6R (rw) register accessor: Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details.

You can [`read`](crate::Reg::read) this register and get [`csgcm6r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcm6r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#CRYP1:CSGCM6R)

For information about available fields see [`mod@csgcm6r`] module*/
pub type CSGCM6R = crate::Reg<csgcm6r::CSGCM6Rrs>;
///Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details.
pub mod csgcm6r;
/**CSGCM7R (rw) register accessor: Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details.

You can [`read`](crate::Reg::read) this register and get [`csgcm7r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcm7r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#CRYP1:CSGCM7R)

For information about available fields see [`mod@csgcm7r`] module*/
pub type CSGCM7R = crate::Reg<csgcm7r::CSGCM7Rrs>;
///Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details.
pub mod csgcm7r;
/**HWCFGR (r) register accessor: CRYP hardware configuration register

You can [`read`](crate::Reg::read) this register and get [`hwcfgr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#CRYP1:HWCFGR)

For information about available fields see [`mod@hwcfgr`] module*/
pub type HWCFGR = crate::Reg<hwcfgr::HWCFGRrs>;
///CRYP hardware configuration register
pub mod hwcfgr;
/**VERR (r) register accessor: CRYP HW Version Register

You can [`read`](crate::Reg::read) this register and get [`verr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#CRYP1:VERR)

For information about available fields see [`mod@verr`] module*/
pub type VERR = crate::Reg<verr::VERRrs>;
///CRYP HW Version Register
pub mod verr;
/**IPIDR (r) register accessor: CRYP Identification

You can [`read`](crate::Reg::read) this register and get [`ipidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#CRYP1:IPIDR)

For information about available fields see [`mod@ipidr`] module*/
pub type IPIDR = crate::Reg<ipidr::IPIDRrs>;
///CRYP Identification
pub mod ipidr;
/**MID (r) register accessor: CRYP HW Magic ID

You can [`read`](crate::Reg::read) this register and get [`mid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#CRYP1:MID)

For information about available fields see [`mod@mid`] module*/
pub type MID = crate::Reg<mid::MIDrs>;
///CRYP HW Magic ID
pub mod mid;
