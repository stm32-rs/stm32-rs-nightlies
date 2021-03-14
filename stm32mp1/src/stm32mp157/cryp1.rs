#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CRYP control register"]
    pub cryp_cr: CRYP_CR,
    #[doc = "0x04 - CRYP status register"]
    pub cryp_sr: CRYP_SR,
    #[doc = "0x08 - The CRYP_DIN register is the data input register. It is 32-bit wide. It is used to enter into the input FIFO up to four 64-bit blocks (TDES) or two 128-bit blocks (AES) of plaintext (when encrypting) or ciphertext (when decrypting), one 32-bit word at a time. To fit different data sizes, the data can be swapped after processing by configuring the DATATYPE bits in the CRYP_CR register. Refer to Section39.3.16: CRYP data registers and data swapping for more details. When CRYP_DIN register is written to the data are pushed into the input FIFO. If CRYPEN = 1, when at least two 32-bit words in the DES/TDES mode have been pushed into the input FIFO (four words in the AES mode), and when at least two words are free in the output FIFO (four words in the AES mode), the CRYP engine starts an encrypting or decrypting process. When CRYP_DIN register is read: If CRYPEN = 0, the FIFO is popped, and then the data present in the Input FIFO are returned, from the oldest one (first reading) to the newest one (last reading). The IFEM flag must be checked before each read operation to make sure that the FIFO is not empty. if CRYPEN = 1, an undefined value is returned. After the CRYP_DIN register has been read once or several times, the FIFO must be flushed by setting the FFLUSH bit prior to processing new data."]
    pub cryp_din: CRYP_DIN,
    #[doc = "0x0c - The CRYP_DOUT register is the data output register. It is read-only and 32-bit wide. It is used to retrieve from the output FIFO up to four 64-bit blocks (TDES) or two 128-bit blocks (AES) of plaintext (when encrypting) or ciphertext (when decrypting), one 32-bit word at a time. To fit different data sizes, the data can be swapped after processing by configuring the DATATYPE bits in the CRYP_CR register. Refer to Section39.3.16: CRYP data registers and data swapping for more details. When CRYP_DOUT register is read, the last data entered into the output FIFO (pointed to by the read pointer) is returned."]
    pub cryp_dout: CRYP_DOUT,
    #[doc = "0x10 - CRYP DMA control register"]
    pub cryp_dmacr: CRYP_DMACR,
    #[doc = "0x14 - The CRYP_IMSCR register is the interrupt mask set or clear register. It is a read/write register. When a read operation is performed, this register gives the current value of the mask applied to the relevant interrupt. Writing 1 to the particular bit sets the mask, thus enabling the interrupt to be read. Writing 0 to this bit clears the corresponding mask. All the bits are cleared to 0 when the peripheral is reset."]
    pub cryp_imscr: CRYP_IMSCR,
    #[doc = "0x18 - The CRYP_RISR register is the raw interrupt status register. It is a read-only register. When a read operation is performed, this register gives the current raw status of the corresponding interrupt, i.e. the interrupt information without taking CRYP_IMSCR mask into account. Write operations have no effect."]
    pub cryp_risr: CRYP_RISR,
    #[doc = "0x1c - The CRYP_MISR register is the masked interrupt status register. It is a read-only register. When a read operation is performed, this register gives the current masked status of the corresponding interrupt, i.e. the interrupt information taking CRYP_IMSCR mask into account. Write operations have no effect."]
    pub cryp_misr: CRYP_MISR,
    #[doc = "0x20 - CRYP key registers contain the cryptographic keys. In DES/TDES mode, the keys are 64-bit binary values (number from left to right, that is the leftmost bit is bit 1) and named K1, K2 and K3 (K0 is not used). Each key consists of 56 information bits and 8 parity bits. In AES mode, the key is considered as a single 128, 192 or 256 bits long sequence K0K1K2...K127/191/255. The AES key is entered into the registers as follows: for AES-128: K0..K127 corresponds to b127..b0 (b255..b128 are not used), for AES-192: K0..K191 corresponds to b191..b0 (b255..b192 are not used), for AES-256: K0..K255 corresponds to b255..b0. In all cases key bit K0 is the leftmost bit in CRYP inner memory and register bit b0 is the rightmost bit in corresponding CRYP_KxLR key register. For more information refer to Section39.3.17: CRYP key registers. Write accesses to these registers are disregarded when the cryptographic processor is busy (bit BUSY = 1 in the CRYP_SR register)"]
    pub cryp_k0lr: CRYP_K0LR,
    #[doc = "0x24 - Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details."]
    pub cryp_k0rr: CRYP_K0RR,
    #[doc = "0x28 - Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details."]
    pub cryp_k1lr: CRYP_K1LR,
    #[doc = "0x2c - Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details."]
    pub cryp_k1rr: CRYP_K1RR,
    #[doc = "0x30 - Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details."]
    pub cryp_k2lr: CRYP_K2LR,
    #[doc = "0x34 - Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details."]
    pub cryp_k2rr: CRYP_K2RR,
    #[doc = "0x38 - Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details."]
    pub cryp_k3lr: CRYP_K3LR,
    #[doc = "0x3c - Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details."]
    pub cryp_k3rr: CRYP_K3RR,
    #[doc = "0x40 - The CRYP_IV0...1(L/R)R are the left-word and right-word registers for the initialization vector (64 bits for DES/TDES and 128 bits for AES). For more information refer to Section39.3.18: CRYP initialization vector registers. IV0 is the leftmost bit whereas IV63 (DES, TDES) or IV127 (AES) are the rightmost bits of the initialization vector. IV1(L/R)R is used only in the AES. Only CRYP_IV0(L/R) is used in DES/TDES. Write access to these registers are disregarded when the cryptographic processor is busy (bit BUSY = 1 in the CRYP_SR register)."]
    pub cryp_iv0lr: CRYP_IV0LR,
    #[doc = "0x44 - Refer to Section39.6.17: CRYP initialization vector register 0L (CRYP_IV0LR) for details."]
    pub cryp_iv0rr: CRYP_IV0RR,
    #[doc = "0x48 - Refer to Section39.6.17: CRYP initialization vector register 0L (CRYP_IV0LR) for details."]
    pub cryp_iv1lr: CRYP_IV1LR,
    #[doc = "0x4c - Refer to Section39.6.17: CRYP initialization vector register 0L (CRYP_IV0LR) for details."]
    pub cryp_iv1rr: CRYP_IV1RR,
    #[doc = "0x50 - These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers."]
    pub cryp_csgcmccm0r: CRYP_CSGCMCCM0R,
    #[doc = "0x54 - These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers."]
    pub cryp_csgcmccm1r: CRYP_CSGCMCCM1R,
    #[doc = "0x58 - These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers."]
    pub cryp_csgcmccm2r: CRYP_CSGCMCCM2R,
    #[doc = "0x5c - These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers."]
    pub cryp_csgcmccm3r: CRYP_CSGCMCCM3R,
    #[doc = "0x60 - These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers."]
    pub cryp_csgcmccm4r: CRYP_CSGCMCCM4R,
    #[doc = "0x64 - These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers."]
    pub cryp_csgcmccm5r: CRYP_CSGCMCCM5R,
    #[doc = "0x68 - These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers."]
    pub cryp_csgcmccm6r: CRYP_CSGCMCCM6R,
    #[doc = "0x6c - These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers."]
    pub cryp_csgcmccm7r: CRYP_CSGCMCCM7R,
    #[doc = "0x70 - Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details."]
    pub cryp_csgcm0r: CRYP_CSGCM0R,
    #[doc = "0x74 - Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details."]
    pub cryp_csgcm1r: CRYP_CSGCM1R,
    #[doc = "0x78 - Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details."]
    pub cryp_csgcm2r: CRYP_CSGCM2R,
    #[doc = "0x7c - Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details."]
    pub cryp_csgcm3r: CRYP_CSGCM3R,
    #[doc = "0x80 - Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details."]
    pub cryp_csgcm4r: CRYP_CSGCM4R,
    #[doc = "0x84 - Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details."]
    pub cryp_csgcm5r: CRYP_CSGCM5R,
    #[doc = "0x88 - Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details."]
    pub cryp_csgcm6r: CRYP_CSGCM6R,
    #[doc = "0x8c - Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details."]
    pub cryp_csgcm7r: CRYP_CSGCM7R,
    _reserved36: [u8; 864usize],
    #[doc = "0x3f0 - CRYP hardware configuration register"]
    pub cryp_hwcfgr: CRYP_HWCFGR,
    #[doc = "0x3f4 - CRYP HW Version Register"]
    pub cryp_verr: CRYP_VERR,
    #[doc = "0x3f8 - CRYP Identification"]
    pub cryp_ipidr: CRYP_IPIDR,
    #[doc = "0x3fc - CRYP HW Magic ID"]
    pub cryp_mid: CRYP_MID,
}
#[doc = "CRYP control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cryp_cr](cryp_cr) module"]
pub type CRYP_CR = crate::Reg<u32, _CRYP_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRYP_CR;
#[doc = "`read()` method returns [cryp_cr::R](cryp_cr::R) reader structure"]
impl crate::Readable for CRYP_CR {}
#[doc = "`write(|w| ..)` method takes [cryp_cr::W](cryp_cr::W) writer structure"]
impl crate::Writable for CRYP_CR {}
#[doc = "CRYP control register"]
pub mod cryp_cr;
#[doc = "CRYP status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cryp_sr](cryp_sr) module"]
pub type CRYP_SR = crate::Reg<u32, _CRYP_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRYP_SR;
#[doc = "`read()` method returns [cryp_sr::R](cryp_sr::R) reader structure"]
impl crate::Readable for CRYP_SR {}
#[doc = "CRYP status register"]
pub mod cryp_sr;
#[doc = "The CRYP_DIN register is the data input register. It is 32-bit wide. It is used to enter into the input FIFO up to four 64-bit blocks (TDES) or two 128-bit blocks (AES) of plaintext (when encrypting) or ciphertext (when decrypting), one 32-bit word at a time. To fit different data sizes, the data can be swapped after processing by configuring the DATATYPE bits in the CRYP_CR register. Refer to Section39.3.16: CRYP data registers and data swapping for more details. When CRYP_DIN register is written to the data are pushed into the input FIFO. If CRYPEN = 1, when at least two 32-bit words in the DES/TDES mode have been pushed into the input FIFO (four words in the AES mode), and when at least two words are free in the output FIFO (four words in the AES mode), the CRYP engine starts an encrypting or decrypting process. When CRYP_DIN register is read: If CRYPEN = 0, the FIFO is popped, and then the data present in the Input FIFO are returned, from the oldest one (first reading) to the newest one (last reading). The IFEM flag must be checked before each read operation to make sure that the FIFO is not empty. if CRYPEN = 1, an undefined value is returned. After the CRYP_DIN register has been read once or several times, the FIFO must be flushed by setting the FFLUSH bit prior to processing new data.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cryp_din](cryp_din) module"]
pub type CRYP_DIN = crate::Reg<u32, _CRYP_DIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRYP_DIN;
#[doc = "`read()` method returns [cryp_din::R](cryp_din::R) reader structure"]
impl crate::Readable for CRYP_DIN {}
#[doc = "`write(|w| ..)` method takes [cryp_din::W](cryp_din::W) writer structure"]
impl crate::Writable for CRYP_DIN {}
#[doc = "The CRYP_DIN register is the data input register. It is 32-bit wide. It is used to enter into the input FIFO up to four 64-bit blocks (TDES) or two 128-bit blocks (AES) of plaintext (when encrypting) or ciphertext (when decrypting), one 32-bit word at a time. To fit different data sizes, the data can be swapped after processing by configuring the DATATYPE bits in the CRYP_CR register. Refer to Section39.3.16: CRYP data registers and data swapping for more details. When CRYP_DIN register is written to the data are pushed into the input FIFO. If CRYPEN = 1, when at least two 32-bit words in the DES/TDES mode have been pushed into the input FIFO (four words in the AES mode), and when at least two words are free in the output FIFO (four words in the AES mode), the CRYP engine starts an encrypting or decrypting process. When CRYP_DIN register is read: If CRYPEN = 0, the FIFO is popped, and then the data present in the Input FIFO are returned, from the oldest one (first reading) to the newest one (last reading). The IFEM flag must be checked before each read operation to make sure that the FIFO is not empty. if CRYPEN = 1, an undefined value is returned. After the CRYP_DIN register has been read once or several times, the FIFO must be flushed by setting the FFLUSH bit prior to processing new data."]
pub mod cryp_din;
#[doc = "The CRYP_DOUT register is the data output register. It is read-only and 32-bit wide. It is used to retrieve from the output FIFO up to four 64-bit blocks (TDES) or two 128-bit blocks (AES) of plaintext (when encrypting) or ciphertext (when decrypting), one 32-bit word at a time. To fit different data sizes, the data can be swapped after processing by configuring the DATATYPE bits in the CRYP_CR register. Refer to Section39.3.16: CRYP data registers and data swapping for more details. When CRYP_DOUT register is read, the last data entered into the output FIFO (pointed to by the read pointer) is returned.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cryp_dout](cryp_dout) module"]
pub type CRYP_DOUT = crate::Reg<u32, _CRYP_DOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRYP_DOUT;
#[doc = "`read()` method returns [cryp_dout::R](cryp_dout::R) reader structure"]
impl crate::Readable for CRYP_DOUT {}
#[doc = "The CRYP_DOUT register is the data output register. It is read-only and 32-bit wide. It is used to retrieve from the output FIFO up to four 64-bit blocks (TDES) or two 128-bit blocks (AES) of plaintext (when encrypting) or ciphertext (when decrypting), one 32-bit word at a time. To fit different data sizes, the data can be swapped after processing by configuring the DATATYPE bits in the CRYP_CR register. Refer to Section39.3.16: CRYP data registers and data swapping for more details. When CRYP_DOUT register is read, the last data entered into the output FIFO (pointed to by the read pointer) is returned."]
pub mod cryp_dout;
#[doc = "CRYP DMA control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cryp_dmacr](cryp_dmacr) module"]
pub type CRYP_DMACR = crate::Reg<u32, _CRYP_DMACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRYP_DMACR;
#[doc = "`read()` method returns [cryp_dmacr::R](cryp_dmacr::R) reader structure"]
impl crate::Readable for CRYP_DMACR {}
#[doc = "`write(|w| ..)` method takes [cryp_dmacr::W](cryp_dmacr::W) writer structure"]
impl crate::Writable for CRYP_DMACR {}
#[doc = "CRYP DMA control register"]
pub mod cryp_dmacr;
#[doc = "The CRYP_IMSCR register is the interrupt mask set or clear register. It is a read/write register. When a read operation is performed, this register gives the current value of the mask applied to the relevant interrupt. Writing 1 to the particular bit sets the mask, thus enabling the interrupt to be read. Writing 0 to this bit clears the corresponding mask. All the bits are cleared to 0 when the peripheral is reset.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cryp_imscr](cryp_imscr) module"]
pub type CRYP_IMSCR = crate::Reg<u32, _CRYP_IMSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRYP_IMSCR;
#[doc = "`read()` method returns [cryp_imscr::R](cryp_imscr::R) reader structure"]
impl crate::Readable for CRYP_IMSCR {}
#[doc = "`write(|w| ..)` method takes [cryp_imscr::W](cryp_imscr::W) writer structure"]
impl crate::Writable for CRYP_IMSCR {}
#[doc = "The CRYP_IMSCR register is the interrupt mask set or clear register. It is a read/write register. When a read operation is performed, this register gives the current value of the mask applied to the relevant interrupt. Writing 1 to the particular bit sets the mask, thus enabling the interrupt to be read. Writing 0 to this bit clears the corresponding mask. All the bits are cleared to 0 when the peripheral is reset."]
pub mod cryp_imscr;
#[doc = "The CRYP_RISR register is the raw interrupt status register. It is a read-only register. When a read operation is performed, this register gives the current raw status of the corresponding interrupt, i.e. the interrupt information without taking CRYP_IMSCR mask into account. Write operations have no effect.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cryp_risr](cryp_risr) module"]
pub type CRYP_RISR = crate::Reg<u32, _CRYP_RISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRYP_RISR;
#[doc = "`read()` method returns [cryp_risr::R](cryp_risr::R) reader structure"]
impl crate::Readable for CRYP_RISR {}
#[doc = "The CRYP_RISR register is the raw interrupt status register. It is a read-only register. When a read operation is performed, this register gives the current raw status of the corresponding interrupt, i.e. the interrupt information without taking CRYP_IMSCR mask into account. Write operations have no effect."]
pub mod cryp_risr;
#[doc = "The CRYP_MISR register is the masked interrupt status register. It is a read-only register. When a read operation is performed, this register gives the current masked status of the corresponding interrupt, i.e. the interrupt information taking CRYP_IMSCR mask into account. Write operations have no effect.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cryp_misr](cryp_misr) module"]
pub type CRYP_MISR = crate::Reg<u32, _CRYP_MISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRYP_MISR;
#[doc = "`read()` method returns [cryp_misr::R](cryp_misr::R) reader structure"]
impl crate::Readable for CRYP_MISR {}
#[doc = "The CRYP_MISR register is the masked interrupt status register. It is a read-only register. When a read operation is performed, this register gives the current masked status of the corresponding interrupt, i.e. the interrupt information taking CRYP_IMSCR mask into account. Write operations have no effect."]
pub mod cryp_misr;
#[doc = "CRYP key registers contain the cryptographic keys. In DES/TDES mode, the keys are 64-bit binary values (number from left to right, that is the leftmost bit is bit 1) and named K1, K2 and K3 (K0 is not used). Each key consists of 56 information bits and 8 parity bits. In AES mode, the key is considered as a single 128, 192 or 256 bits long sequence K0K1K2...K127/191/255. The AES key is entered into the registers as follows: for AES-128: K0..K127 corresponds to b127..b0 (b255..b128 are not used), for AES-192: K0..K191 corresponds to b191..b0 (b255..b192 are not used), for AES-256: K0..K255 corresponds to b255..b0. In all cases key bit K0 is the leftmost bit in CRYP inner memory and register bit b0 is the rightmost bit in corresponding CRYP_KxLR key register. For more information refer to Section39.3.17: CRYP key registers. Write accesses to these registers are disregarded when the cryptographic processor is busy (bit BUSY = 1 in the CRYP_SR register)\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cryp_k0lr](cryp_k0lr) module"]
pub type CRYP_K0LR = crate::Reg<u32, _CRYP_K0LR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRYP_K0LR;
#[doc = "`write(|w| ..)` method takes [cryp_k0lr::W](cryp_k0lr::W) writer structure"]
impl crate::Writable for CRYP_K0LR {}
#[doc = "CRYP key registers contain the cryptographic keys. In DES/TDES mode, the keys are 64-bit binary values (number from left to right, that is the leftmost bit is bit 1) and named K1, K2 and K3 (K0 is not used). Each key consists of 56 information bits and 8 parity bits. In AES mode, the key is considered as a single 128, 192 or 256 bits long sequence K0K1K2...K127/191/255. The AES key is entered into the registers as follows: for AES-128: K0..K127 corresponds to b127..b0 (b255..b128 are not used), for AES-192: K0..K191 corresponds to b191..b0 (b255..b192 are not used), for AES-256: K0..K255 corresponds to b255..b0. In all cases key bit K0 is the leftmost bit in CRYP inner memory and register bit b0 is the rightmost bit in corresponding CRYP_KxLR key register. For more information refer to Section39.3.17: CRYP key registers. Write accesses to these registers are disregarded when the cryptographic processor is busy (bit BUSY = 1 in the CRYP_SR register)"]
pub mod cryp_k0lr;
#[doc = "Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cryp_k0rr](cryp_k0rr) module"]
pub type CRYP_K0RR = crate::Reg<u32, _CRYP_K0RR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRYP_K0RR;
#[doc = "`write(|w| ..)` method takes [cryp_k0rr::W](cryp_k0rr::W) writer structure"]
impl crate::Writable for CRYP_K0RR {}
#[doc = "Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details."]
pub mod cryp_k0rr;
#[doc = "Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cryp_k1lr](cryp_k1lr) module"]
pub type CRYP_K1LR = crate::Reg<u32, _CRYP_K1LR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRYP_K1LR;
#[doc = "`write(|w| ..)` method takes [cryp_k1lr::W](cryp_k1lr::W) writer structure"]
impl crate::Writable for CRYP_K1LR {}
#[doc = "Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details."]
pub mod cryp_k1lr;
#[doc = "Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cryp_k1rr](cryp_k1rr) module"]
pub type CRYP_K1RR = crate::Reg<u32, _CRYP_K1RR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRYP_K1RR;
#[doc = "`write(|w| ..)` method takes [cryp_k1rr::W](cryp_k1rr::W) writer structure"]
impl crate::Writable for CRYP_K1RR {}
#[doc = "Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details."]
pub mod cryp_k1rr;
#[doc = "Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cryp_k2lr](cryp_k2lr) module"]
pub type CRYP_K2LR = crate::Reg<u32, _CRYP_K2LR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRYP_K2LR;
#[doc = "`write(|w| ..)` method takes [cryp_k2lr::W](cryp_k2lr::W) writer structure"]
impl crate::Writable for CRYP_K2LR {}
#[doc = "Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details."]
pub mod cryp_k2lr;
#[doc = "Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cryp_k2rr](cryp_k2rr) module"]
pub type CRYP_K2RR = crate::Reg<u32, _CRYP_K2RR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRYP_K2RR;
#[doc = "`write(|w| ..)` method takes [cryp_k2rr::W](cryp_k2rr::W) writer structure"]
impl crate::Writable for CRYP_K2RR {}
#[doc = "Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details."]
pub mod cryp_k2rr;
#[doc = "Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cryp_k3lr](cryp_k3lr) module"]
pub type CRYP_K3LR = crate::Reg<u32, _CRYP_K3LR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRYP_K3LR;
#[doc = "`write(|w| ..)` method takes [cryp_k3lr::W](cryp_k3lr::W) writer structure"]
impl crate::Writable for CRYP_K3LR {}
#[doc = "Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details."]
pub mod cryp_k3lr;
#[doc = "Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cryp_k3rr](cryp_k3rr) module"]
pub type CRYP_K3RR = crate::Reg<u32, _CRYP_K3RR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRYP_K3RR;
#[doc = "`write(|w| ..)` method takes [cryp_k3rr::W](cryp_k3rr::W) writer structure"]
impl crate::Writable for CRYP_K3RR {}
#[doc = "Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details."]
pub mod cryp_k3rr;
#[doc = "The CRYP_IV0...1(L/R)R are the left-word and right-word registers for the initialization vector (64 bits for DES/TDES and 128 bits for AES). For more information refer to Section39.3.18: CRYP initialization vector registers. IV0 is the leftmost bit whereas IV63 (DES, TDES) or IV127 (AES) are the rightmost bits of the initialization vector. IV1(L/R)R is used only in the AES. Only CRYP_IV0(L/R) is used in DES/TDES. Write access to these registers are disregarded when the cryptographic processor is busy (bit BUSY = 1 in the CRYP_SR register).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cryp_iv0lr](cryp_iv0lr) module"]
pub type CRYP_IV0LR = crate::Reg<u32, _CRYP_IV0LR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRYP_IV0LR;
#[doc = "`read()` method returns [cryp_iv0lr::R](cryp_iv0lr::R) reader structure"]
impl crate::Readable for CRYP_IV0LR {}
#[doc = "`write(|w| ..)` method takes [cryp_iv0lr::W](cryp_iv0lr::W) writer structure"]
impl crate::Writable for CRYP_IV0LR {}
#[doc = "The CRYP_IV0...1(L/R)R are the left-word and right-word registers for the initialization vector (64 bits for DES/TDES and 128 bits for AES). For more information refer to Section39.3.18: CRYP initialization vector registers. IV0 is the leftmost bit whereas IV63 (DES, TDES) or IV127 (AES) are the rightmost bits of the initialization vector. IV1(L/R)R is used only in the AES. Only CRYP_IV0(L/R) is used in DES/TDES. Write access to these registers are disregarded when the cryptographic processor is busy (bit BUSY = 1 in the CRYP_SR register)."]
pub mod cryp_iv0lr;
#[doc = "Refer to Section39.6.17: CRYP initialization vector register 0L (CRYP_IV0LR) for details.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cryp_iv0rr](cryp_iv0rr) module"]
pub type CRYP_IV0RR = crate::Reg<u32, _CRYP_IV0RR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRYP_IV0RR;
#[doc = "`read()` method returns [cryp_iv0rr::R](cryp_iv0rr::R) reader structure"]
impl crate::Readable for CRYP_IV0RR {}
#[doc = "`write(|w| ..)` method takes [cryp_iv0rr::W](cryp_iv0rr::W) writer structure"]
impl crate::Writable for CRYP_IV0RR {}
#[doc = "Refer to Section39.6.17: CRYP initialization vector register 0L (CRYP_IV0LR) for details."]
pub mod cryp_iv0rr;
#[doc = "Refer to Section39.6.17: CRYP initialization vector register 0L (CRYP_IV0LR) for details.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cryp_iv1lr](cryp_iv1lr) module"]
pub type CRYP_IV1LR = crate::Reg<u32, _CRYP_IV1LR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRYP_IV1LR;
#[doc = "`read()` method returns [cryp_iv1lr::R](cryp_iv1lr::R) reader structure"]
impl crate::Readable for CRYP_IV1LR {}
#[doc = "`write(|w| ..)` method takes [cryp_iv1lr::W](cryp_iv1lr::W) writer structure"]
impl crate::Writable for CRYP_IV1LR {}
#[doc = "Refer to Section39.6.17: CRYP initialization vector register 0L (CRYP_IV0LR) for details."]
pub mod cryp_iv1lr;
#[doc = "Refer to Section39.6.17: CRYP initialization vector register 0L (CRYP_IV0LR) for details.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cryp_iv1rr](cryp_iv1rr) module"]
pub type CRYP_IV1RR = crate::Reg<u32, _CRYP_IV1RR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRYP_IV1RR;
#[doc = "`read()` method returns [cryp_iv1rr::R](cryp_iv1rr::R) reader structure"]
impl crate::Readable for CRYP_IV1RR {}
#[doc = "`write(|w| ..)` method takes [cryp_iv1rr::W](cryp_iv1rr::W) writer structure"]
impl crate::Writable for CRYP_IV1RR {}
#[doc = "Refer to Section39.6.17: CRYP initialization vector register 0L (CRYP_IV0LR) for details."]
pub mod cryp_iv1rr;
#[doc = "These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cryp_csgcmccm0r](cryp_csgcmccm0r) module"]
pub type CRYP_CSGCMCCM0R = crate::Reg<u32, _CRYP_CSGCMCCM0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRYP_CSGCMCCM0R;
#[doc = "`read()` method returns [cryp_csgcmccm0r::R](cryp_csgcmccm0r::R) reader structure"]
impl crate::Readable for CRYP_CSGCMCCM0R {}
#[doc = "`write(|w| ..)` method takes [cryp_csgcmccm0r::W](cryp_csgcmccm0r::W) writer structure"]
impl crate::Writable for CRYP_CSGCMCCM0R {}
#[doc = "These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers."]
pub mod cryp_csgcmccm0r;
#[doc = "These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cryp_csgcmccm1r](cryp_csgcmccm1r) module"]
pub type CRYP_CSGCMCCM1R = crate::Reg<u32, _CRYP_CSGCMCCM1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRYP_CSGCMCCM1R;
#[doc = "`read()` method returns [cryp_csgcmccm1r::R](cryp_csgcmccm1r::R) reader structure"]
impl crate::Readable for CRYP_CSGCMCCM1R {}
#[doc = "`write(|w| ..)` method takes [cryp_csgcmccm1r::W](cryp_csgcmccm1r::W) writer structure"]
impl crate::Writable for CRYP_CSGCMCCM1R {}
#[doc = "These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers."]
pub mod cryp_csgcmccm1r;
#[doc = "These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cryp_csgcmccm2r](cryp_csgcmccm2r) module"]
pub type CRYP_CSGCMCCM2R = crate::Reg<u32, _CRYP_CSGCMCCM2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRYP_CSGCMCCM2R;
#[doc = "`read()` method returns [cryp_csgcmccm2r::R](cryp_csgcmccm2r::R) reader structure"]
impl crate::Readable for CRYP_CSGCMCCM2R {}
#[doc = "`write(|w| ..)` method takes [cryp_csgcmccm2r::W](cryp_csgcmccm2r::W) writer structure"]
impl crate::Writable for CRYP_CSGCMCCM2R {}
#[doc = "These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers."]
pub mod cryp_csgcmccm2r;
#[doc = "These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cryp_csgcmccm3r](cryp_csgcmccm3r) module"]
pub type CRYP_CSGCMCCM3R = crate::Reg<u32, _CRYP_CSGCMCCM3R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRYP_CSGCMCCM3R;
#[doc = "`read()` method returns [cryp_csgcmccm3r::R](cryp_csgcmccm3r::R) reader structure"]
impl crate::Readable for CRYP_CSGCMCCM3R {}
#[doc = "`write(|w| ..)` method takes [cryp_csgcmccm3r::W](cryp_csgcmccm3r::W) writer structure"]
impl crate::Writable for CRYP_CSGCMCCM3R {}
#[doc = "These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers."]
pub mod cryp_csgcmccm3r;
#[doc = "These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cryp_csgcmccm4r](cryp_csgcmccm4r) module"]
pub type CRYP_CSGCMCCM4R = crate::Reg<u32, _CRYP_CSGCMCCM4R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRYP_CSGCMCCM4R;
#[doc = "`read()` method returns [cryp_csgcmccm4r::R](cryp_csgcmccm4r::R) reader structure"]
impl crate::Readable for CRYP_CSGCMCCM4R {}
#[doc = "`write(|w| ..)` method takes [cryp_csgcmccm4r::W](cryp_csgcmccm4r::W) writer structure"]
impl crate::Writable for CRYP_CSGCMCCM4R {}
#[doc = "These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers."]
pub mod cryp_csgcmccm4r;
#[doc = "These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cryp_csgcmccm5r](cryp_csgcmccm5r) module"]
pub type CRYP_CSGCMCCM5R = crate::Reg<u32, _CRYP_CSGCMCCM5R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRYP_CSGCMCCM5R;
#[doc = "`read()` method returns [cryp_csgcmccm5r::R](cryp_csgcmccm5r::R) reader structure"]
impl crate::Readable for CRYP_CSGCMCCM5R {}
#[doc = "`write(|w| ..)` method takes [cryp_csgcmccm5r::W](cryp_csgcmccm5r::W) writer structure"]
impl crate::Writable for CRYP_CSGCMCCM5R {}
#[doc = "These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers."]
pub mod cryp_csgcmccm5r;
#[doc = "These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cryp_csgcmccm6r](cryp_csgcmccm6r) module"]
pub type CRYP_CSGCMCCM6R = crate::Reg<u32, _CRYP_CSGCMCCM6R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRYP_CSGCMCCM6R;
#[doc = "`read()` method returns [cryp_csgcmccm6r::R](cryp_csgcmccm6r::R) reader structure"]
impl crate::Readable for CRYP_CSGCMCCM6R {}
#[doc = "`write(|w| ..)` method takes [cryp_csgcmccm6r::W](cryp_csgcmccm6r::W) writer structure"]
impl crate::Writable for CRYP_CSGCMCCM6R {}
#[doc = "These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers."]
pub mod cryp_csgcmccm6r;
#[doc = "These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cryp_csgcmccm7r](cryp_csgcmccm7r) module"]
pub type CRYP_CSGCMCCM7R = crate::Reg<u32, _CRYP_CSGCMCCM7R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRYP_CSGCMCCM7R;
#[doc = "`read()` method returns [cryp_csgcmccm7r::R](cryp_csgcmccm7r::R) reader structure"]
impl crate::Readable for CRYP_CSGCMCCM7R {}
#[doc = "`write(|w| ..)` method takes [cryp_csgcmccm7r::W](cryp_csgcmccm7r::W) writer structure"]
impl crate::Writable for CRYP_CSGCMCCM7R {}
#[doc = "These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers."]
pub mod cryp_csgcmccm7r;
#[doc = "Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cryp_csgcm0r](cryp_csgcm0r) module"]
pub type CRYP_CSGCM0R = crate::Reg<u32, _CRYP_CSGCM0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRYP_CSGCM0R;
#[doc = "`read()` method returns [cryp_csgcm0r::R](cryp_csgcm0r::R) reader structure"]
impl crate::Readable for CRYP_CSGCM0R {}
#[doc = "`write(|w| ..)` method takes [cryp_csgcm0r::W](cryp_csgcm0r::W) writer structure"]
impl crate::Writable for CRYP_CSGCM0R {}
#[doc = "Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details."]
pub mod cryp_csgcm0r;
#[doc = "Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cryp_csgcm1r](cryp_csgcm1r) module"]
pub type CRYP_CSGCM1R = crate::Reg<u32, _CRYP_CSGCM1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRYP_CSGCM1R;
#[doc = "`read()` method returns [cryp_csgcm1r::R](cryp_csgcm1r::R) reader structure"]
impl crate::Readable for CRYP_CSGCM1R {}
#[doc = "`write(|w| ..)` method takes [cryp_csgcm1r::W](cryp_csgcm1r::W) writer structure"]
impl crate::Writable for CRYP_CSGCM1R {}
#[doc = "Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details."]
pub mod cryp_csgcm1r;
#[doc = "Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cryp_csgcm2r](cryp_csgcm2r) module"]
pub type CRYP_CSGCM2R = crate::Reg<u32, _CRYP_CSGCM2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRYP_CSGCM2R;
#[doc = "`read()` method returns [cryp_csgcm2r::R](cryp_csgcm2r::R) reader structure"]
impl crate::Readable for CRYP_CSGCM2R {}
#[doc = "`write(|w| ..)` method takes [cryp_csgcm2r::W](cryp_csgcm2r::W) writer structure"]
impl crate::Writable for CRYP_CSGCM2R {}
#[doc = "Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details."]
pub mod cryp_csgcm2r;
#[doc = "Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cryp_csgcm3r](cryp_csgcm3r) module"]
pub type CRYP_CSGCM3R = crate::Reg<u32, _CRYP_CSGCM3R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRYP_CSGCM3R;
#[doc = "`read()` method returns [cryp_csgcm3r::R](cryp_csgcm3r::R) reader structure"]
impl crate::Readable for CRYP_CSGCM3R {}
#[doc = "`write(|w| ..)` method takes [cryp_csgcm3r::W](cryp_csgcm3r::W) writer structure"]
impl crate::Writable for CRYP_CSGCM3R {}
#[doc = "Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details."]
pub mod cryp_csgcm3r;
#[doc = "Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cryp_csgcm4r](cryp_csgcm4r) module"]
pub type CRYP_CSGCM4R = crate::Reg<u32, _CRYP_CSGCM4R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRYP_CSGCM4R;
#[doc = "`read()` method returns [cryp_csgcm4r::R](cryp_csgcm4r::R) reader structure"]
impl crate::Readable for CRYP_CSGCM4R {}
#[doc = "`write(|w| ..)` method takes [cryp_csgcm4r::W](cryp_csgcm4r::W) writer structure"]
impl crate::Writable for CRYP_CSGCM4R {}
#[doc = "Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details."]
pub mod cryp_csgcm4r;
#[doc = "Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cryp_csgcm5r](cryp_csgcm5r) module"]
pub type CRYP_CSGCM5R = crate::Reg<u32, _CRYP_CSGCM5R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRYP_CSGCM5R;
#[doc = "`read()` method returns [cryp_csgcm5r::R](cryp_csgcm5r::R) reader structure"]
impl crate::Readable for CRYP_CSGCM5R {}
#[doc = "`write(|w| ..)` method takes [cryp_csgcm5r::W](cryp_csgcm5r::W) writer structure"]
impl crate::Writable for CRYP_CSGCM5R {}
#[doc = "Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details."]
pub mod cryp_csgcm5r;
#[doc = "Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cryp_csgcm6r](cryp_csgcm6r) module"]
pub type CRYP_CSGCM6R = crate::Reg<u32, _CRYP_CSGCM6R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRYP_CSGCM6R;
#[doc = "`read()` method returns [cryp_csgcm6r::R](cryp_csgcm6r::R) reader structure"]
impl crate::Readable for CRYP_CSGCM6R {}
#[doc = "`write(|w| ..)` method takes [cryp_csgcm6r::W](cryp_csgcm6r::W) writer structure"]
impl crate::Writable for CRYP_CSGCM6R {}
#[doc = "Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details."]
pub mod cryp_csgcm6r;
#[doc = "Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cryp_csgcm7r](cryp_csgcm7r) module"]
pub type CRYP_CSGCM7R = crate::Reg<u32, _CRYP_CSGCM7R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRYP_CSGCM7R;
#[doc = "`read()` method returns [cryp_csgcm7r::R](cryp_csgcm7r::R) reader structure"]
impl crate::Readable for CRYP_CSGCM7R {}
#[doc = "`write(|w| ..)` method takes [cryp_csgcm7r::W](cryp_csgcm7r::W) writer structure"]
impl crate::Writable for CRYP_CSGCM7R {}
#[doc = "Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details."]
pub mod cryp_csgcm7r;
#[doc = "CRYP hardware configuration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cryp_hwcfgr](cryp_hwcfgr) module"]
pub type CRYP_HWCFGR = crate::Reg<u32, _CRYP_HWCFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRYP_HWCFGR;
#[doc = "`read()` method returns [cryp_hwcfgr::R](cryp_hwcfgr::R) reader structure"]
impl crate::Readable for CRYP_HWCFGR {}
#[doc = "CRYP hardware configuration register"]
pub mod cryp_hwcfgr;
#[doc = "CRYP HW Version Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cryp_verr](cryp_verr) module"]
pub type CRYP_VERR = crate::Reg<u32, _CRYP_VERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRYP_VERR;
#[doc = "`read()` method returns [cryp_verr::R](cryp_verr::R) reader structure"]
impl crate::Readable for CRYP_VERR {}
#[doc = "CRYP HW Version Register"]
pub mod cryp_verr;
#[doc = "CRYP Identification\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cryp_ipidr](cryp_ipidr) module"]
pub type CRYP_IPIDR = crate::Reg<u32, _CRYP_IPIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRYP_IPIDR;
#[doc = "`read()` method returns [cryp_ipidr::R](cryp_ipidr::R) reader structure"]
impl crate::Readable for CRYP_IPIDR {}
#[doc = "CRYP Identification"]
pub mod cryp_ipidr;
#[doc = "CRYP HW Magic ID\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cryp_mid](cryp_mid) module"]
pub type CRYP_MID = crate::Reg<u32, _CRYP_MID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRYP_MID;
#[doc = "`read()` method returns [cryp_mid::R](cryp_mid::R) reader structure"]
impl crate::Readable for CRYP_MID {}
#[doc = "CRYP HW Magic ID"]
pub mod cryp_mid;
