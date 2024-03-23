#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cryp_cr: CRYP_CR,
    cryp_sr: CRYP_SR,
    cryp_din: CRYP_DIN,
    cryp_dout: CRYP_DOUT,
    cryp_dmacr: CRYP_DMACR,
    cryp_imscr: CRYP_IMSCR,
    cryp_risr: CRYP_RISR,
    cryp_misr: CRYP_MISR,
    cryp_k0lr: CRYP_K0LR,
    cryp_k0rr: CRYP_K0RR,
    cryp_k1lr: CRYP_K1LR,
    cryp_k1rr: CRYP_K1RR,
    cryp_k2lr: CRYP_K2LR,
    cryp_k2rr: CRYP_K2RR,
    cryp_k3lr: CRYP_K3LR,
    cryp_k3rr: CRYP_K3RR,
    cryp_iv0lr: CRYP_IV0LR,
    cryp_iv0rr: CRYP_IV0RR,
    cryp_iv1lr: CRYP_IV1LR,
    cryp_iv1rr: CRYP_IV1RR,
    cryp_csgcmccm0r: CRYP_CSGCMCCM0R,
    cryp_csgcmccm1r: CRYP_CSGCMCCM1R,
    cryp_csgcmccm2r: CRYP_CSGCMCCM2R,
    cryp_csgcmccm3r: CRYP_CSGCMCCM3R,
    cryp_csgcmccm4r: CRYP_CSGCMCCM4R,
    cryp_csgcmccm5r: CRYP_CSGCMCCM5R,
    cryp_csgcmccm6r: CRYP_CSGCMCCM6R,
    cryp_csgcmccm7r: CRYP_CSGCMCCM7R,
    cryp_csgcm0r: CRYP_CSGCM0R,
    cryp_csgcm1r: CRYP_CSGCM1R,
    cryp_csgcm2r: CRYP_CSGCM2R,
    cryp_csgcm3r: CRYP_CSGCM3R,
    cryp_csgcm4r: CRYP_CSGCM4R,
    cryp_csgcm5r: CRYP_CSGCM5R,
    cryp_csgcm6r: CRYP_CSGCM6R,
    cryp_csgcm7r: CRYP_CSGCM7R,
    _reserved36: [u8; 0x0360],
    cryp_hwcfgr: CRYP_HWCFGR,
    cryp_verr: CRYP_VERR,
    cryp_ipidr: CRYP_IPIDR,
    cryp_mid: CRYP_MID,
}
impl RegisterBlock {
    #[doc = "0x00 - CRYP control register"]
    #[inline(always)]
    pub const fn cryp_cr(&self) -> &CRYP_CR {
        &self.cryp_cr
    }
    #[doc = "0x04 - CRYP status register"]
    #[inline(always)]
    pub const fn cryp_sr(&self) -> &CRYP_SR {
        &self.cryp_sr
    }
    #[doc = "0x08 - The CRYP_DIN register is the data input register. It is 32-bit wide. It is used to enter into the input FIFO up to four 64-bit blocks (TDES) or two 128-bit blocks (AES) of plaintext (when encrypting) or ciphertext (when decrypting), one 32-bit word at a time. To fit different data sizes, the data can be swapped after processing by configuring the DATATYPE bits in the CRYP_CR register. Refer to Section39.3.16: CRYP data registers and data swapping for more details. When CRYP_DIN register is written to the data are pushed into the input FIFO. If CRYPEN = 1, when at least two 32-bit words in the DES/TDES mode have been pushed into the input FIFO (four words in the AES mode), and when at least two words are free in the output FIFO (four words in the AES mode), the CRYP engine starts an encrypting or decrypting process. When CRYP_DIN register is read: If CRYPEN = 0, the FIFO is popped, and then the data present in the Input FIFO are returned, from the oldest one (first reading) to the newest one (last reading). The IFEM flag must be checked before each read operation to make sure that the FIFO is not empty. if CRYPEN = 1, an undefined value is returned. After the CRYP_DIN register has been read once or several times, the FIFO must be flushed by setting the FFLUSH bit prior to processing new data."]
    #[inline(always)]
    pub const fn cryp_din(&self) -> &CRYP_DIN {
        &self.cryp_din
    }
    #[doc = "0x0c - The CRYP_DOUT register is the data output register. It is read-only and 32-bit wide. It is used to retrieve from the output FIFO up to four 64-bit blocks (TDES) or two 128-bit blocks (AES) of plaintext (when encrypting) or ciphertext (when decrypting), one 32-bit word at a time. To fit different data sizes, the data can be swapped after processing by configuring the DATATYPE bits in the CRYP_CR register. Refer to Section39.3.16: CRYP data registers and data swapping for more details. When CRYP_DOUT register is read, the last data entered into the output FIFO (pointed to by the read pointer) is returned."]
    #[inline(always)]
    pub const fn cryp_dout(&self) -> &CRYP_DOUT {
        &self.cryp_dout
    }
    #[doc = "0x10 - CRYP DMA control register"]
    #[inline(always)]
    pub const fn cryp_dmacr(&self) -> &CRYP_DMACR {
        &self.cryp_dmacr
    }
    #[doc = "0x14 - The CRYP_IMSCR register is the interrupt mask set or clear register. It is a read/write register. When a read operation is performed, this register gives the current value of the mask applied to the relevant interrupt. Writing 1 to the particular bit sets the mask, thus enabling the interrupt to be read. Writing 0 to this bit clears the corresponding mask. All the bits are cleared to 0 when the peripheral is reset."]
    #[inline(always)]
    pub const fn cryp_imscr(&self) -> &CRYP_IMSCR {
        &self.cryp_imscr
    }
    #[doc = "0x18 - The CRYP_RISR register is the raw interrupt status register. It is a read-only register. When a read operation is performed, this register gives the current raw status of the corresponding interrupt, i.e. the interrupt information without taking CRYP_IMSCR mask into account. Write operations have no effect."]
    #[inline(always)]
    pub const fn cryp_risr(&self) -> &CRYP_RISR {
        &self.cryp_risr
    }
    #[doc = "0x1c - The CRYP_MISR register is the masked interrupt status register. It is a read-only register. When a read operation is performed, this register gives the current masked status of the corresponding interrupt, i.e. the interrupt information taking CRYP_IMSCR mask into account. Write operations have no effect."]
    #[inline(always)]
    pub const fn cryp_misr(&self) -> &CRYP_MISR {
        &self.cryp_misr
    }
    #[doc = "0x20 - CRYP key registers contain the cryptographic keys. In DES/TDES mode, the keys are 64-bit binary values (number from left to right, that is the leftmost bit is bit 1) and named K1, K2 and K3 (K0 is not used). Each key consists of 56 information bits and 8 parity bits. In AES mode, the key is considered as a single 128, 192 or 256 bits long sequence K0K1K2...K127/191/255. The AES key is entered into the registers as follows: for AES-128: K0..K127 corresponds to b127..b0 (b255..b128 are not used), for AES-192: K0..K191 corresponds to b191..b0 (b255..b192 are not used), for AES-256: K0..K255 corresponds to b255..b0. In all cases key bit K0 is the leftmost bit in CRYP inner memory and register bit b0 is the rightmost bit in corresponding CRYP_KxLR key register. For more information refer to Section39.3.17: CRYP key registers. Write accesses to these registers are disregarded when the cryptographic processor is busy (bit BUSY = 1 in the CRYP_SR register)"]
    #[inline(always)]
    pub const fn cryp_k0lr(&self) -> &CRYP_K0LR {
        &self.cryp_k0lr
    }
    #[doc = "0x24 - Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details."]
    #[inline(always)]
    pub const fn cryp_k0rr(&self) -> &CRYP_K0RR {
        &self.cryp_k0rr
    }
    #[doc = "0x28 - Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details."]
    #[inline(always)]
    pub const fn cryp_k1lr(&self) -> &CRYP_K1LR {
        &self.cryp_k1lr
    }
    #[doc = "0x2c - Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details."]
    #[inline(always)]
    pub const fn cryp_k1rr(&self) -> &CRYP_K1RR {
        &self.cryp_k1rr
    }
    #[doc = "0x30 - Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details."]
    #[inline(always)]
    pub const fn cryp_k2lr(&self) -> &CRYP_K2LR {
        &self.cryp_k2lr
    }
    #[doc = "0x34 - Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details."]
    #[inline(always)]
    pub const fn cryp_k2rr(&self) -> &CRYP_K2RR {
        &self.cryp_k2rr
    }
    #[doc = "0x38 - Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details."]
    #[inline(always)]
    pub const fn cryp_k3lr(&self) -> &CRYP_K3LR {
        &self.cryp_k3lr
    }
    #[doc = "0x3c - Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details."]
    #[inline(always)]
    pub const fn cryp_k3rr(&self) -> &CRYP_K3RR {
        &self.cryp_k3rr
    }
    #[doc = "0x40 - The CRYP_IV0...1(L/R)R are the left-word and right-word registers for the initialization vector (64 bits for DES/TDES and 128 bits for AES). For more information refer to Section39.3.18: CRYP initialization vector registers. IV0 is the leftmost bit whereas IV63 (DES, TDES) or IV127 (AES) are the rightmost bits of the initialization vector. IV1(L/R)R is used only in the AES. Only CRYP_IV0(L/R) is used in DES/TDES. Write access to these registers are disregarded when the cryptographic processor is busy (bit BUSY = 1 in the CRYP_SR register)."]
    #[inline(always)]
    pub const fn cryp_iv0lr(&self) -> &CRYP_IV0LR {
        &self.cryp_iv0lr
    }
    #[doc = "0x44 - Refer to Section39.6.17: CRYP initialization vector register 0L (CRYP_IV0LR) for details."]
    #[inline(always)]
    pub const fn cryp_iv0rr(&self) -> &CRYP_IV0RR {
        &self.cryp_iv0rr
    }
    #[doc = "0x48 - Refer to Section39.6.17: CRYP initialization vector register 0L (CRYP_IV0LR) for details."]
    #[inline(always)]
    pub const fn cryp_iv1lr(&self) -> &CRYP_IV1LR {
        &self.cryp_iv1lr
    }
    #[doc = "0x4c - Refer to Section39.6.17: CRYP initialization vector register 0L (CRYP_IV0LR) for details."]
    #[inline(always)]
    pub const fn cryp_iv1rr(&self) -> &CRYP_IV1RR {
        &self.cryp_iv1rr
    }
    #[doc = "0x50 - These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers."]
    #[inline(always)]
    pub const fn cryp_csgcmccm0r(&self) -> &CRYP_CSGCMCCM0R {
        &self.cryp_csgcmccm0r
    }
    #[doc = "0x54 - These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers."]
    #[inline(always)]
    pub const fn cryp_csgcmccm1r(&self) -> &CRYP_CSGCMCCM1R {
        &self.cryp_csgcmccm1r
    }
    #[doc = "0x58 - These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers."]
    #[inline(always)]
    pub const fn cryp_csgcmccm2r(&self) -> &CRYP_CSGCMCCM2R {
        &self.cryp_csgcmccm2r
    }
    #[doc = "0x5c - These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers."]
    #[inline(always)]
    pub const fn cryp_csgcmccm3r(&self) -> &CRYP_CSGCMCCM3R {
        &self.cryp_csgcmccm3r
    }
    #[doc = "0x60 - These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers."]
    #[inline(always)]
    pub const fn cryp_csgcmccm4r(&self) -> &CRYP_CSGCMCCM4R {
        &self.cryp_csgcmccm4r
    }
    #[doc = "0x64 - These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers."]
    #[inline(always)]
    pub const fn cryp_csgcmccm5r(&self) -> &CRYP_CSGCMCCM5R {
        &self.cryp_csgcmccm5r
    }
    #[doc = "0x68 - These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers."]
    #[inline(always)]
    pub const fn cryp_csgcmccm6r(&self) -> &CRYP_CSGCMCCM6R {
        &self.cryp_csgcmccm6r
    }
    #[doc = "0x6c - These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers."]
    #[inline(always)]
    pub const fn cryp_csgcmccm7r(&self) -> &CRYP_CSGCMCCM7R {
        &self.cryp_csgcmccm7r
    }
    #[doc = "0x70 - Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details."]
    #[inline(always)]
    pub const fn cryp_csgcm0r(&self) -> &CRYP_CSGCM0R {
        &self.cryp_csgcm0r
    }
    #[doc = "0x74 - Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details."]
    #[inline(always)]
    pub const fn cryp_csgcm1r(&self) -> &CRYP_CSGCM1R {
        &self.cryp_csgcm1r
    }
    #[doc = "0x78 - Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details."]
    #[inline(always)]
    pub const fn cryp_csgcm2r(&self) -> &CRYP_CSGCM2R {
        &self.cryp_csgcm2r
    }
    #[doc = "0x7c - Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details."]
    #[inline(always)]
    pub const fn cryp_csgcm3r(&self) -> &CRYP_CSGCM3R {
        &self.cryp_csgcm3r
    }
    #[doc = "0x80 - Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details."]
    #[inline(always)]
    pub const fn cryp_csgcm4r(&self) -> &CRYP_CSGCM4R {
        &self.cryp_csgcm4r
    }
    #[doc = "0x84 - Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details."]
    #[inline(always)]
    pub const fn cryp_csgcm5r(&self) -> &CRYP_CSGCM5R {
        &self.cryp_csgcm5r
    }
    #[doc = "0x88 - Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details."]
    #[inline(always)]
    pub const fn cryp_csgcm6r(&self) -> &CRYP_CSGCM6R {
        &self.cryp_csgcm6r
    }
    #[doc = "0x8c - Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details."]
    #[inline(always)]
    pub const fn cryp_csgcm7r(&self) -> &CRYP_CSGCM7R {
        &self.cryp_csgcm7r
    }
    #[doc = "0x3f0 - CRYP hardware configuration register"]
    #[inline(always)]
    pub const fn cryp_hwcfgr(&self) -> &CRYP_HWCFGR {
        &self.cryp_hwcfgr
    }
    #[doc = "0x3f4 - CRYP HW Version Register"]
    #[inline(always)]
    pub const fn cryp_verr(&self) -> &CRYP_VERR {
        &self.cryp_verr
    }
    #[doc = "0x3f8 - CRYP Identification"]
    #[inline(always)]
    pub const fn cryp_ipidr(&self) -> &CRYP_IPIDR {
        &self.cryp_ipidr
    }
    #[doc = "0x3fc - CRYP HW Magic ID"]
    #[inline(always)]
    pub const fn cryp_mid(&self) -> &CRYP_MID {
        &self.cryp_mid
    }
}
#[doc = "CRYP_CR (rw) register accessor: CRYP control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cryp_cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cryp_cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cryp_cr`]
module"]
pub type CRYP_CR = crate::Reg<cryp_cr::CRYP_CRrs>;
#[doc = "CRYP control register"]
pub mod cryp_cr;
#[doc = "CRYP_SR (r) register accessor: CRYP status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cryp_sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cryp_sr`]
module"]
pub type CRYP_SR = crate::Reg<cryp_sr::CRYP_SRrs>;
#[doc = "CRYP status register"]
pub mod cryp_sr;
#[doc = "CRYP_DIN (rw) register accessor: The CRYP_DIN register is the data input register. It is 32-bit wide. It is used to enter into the input FIFO up to four 64-bit blocks (TDES) or two 128-bit blocks (AES) of plaintext (when encrypting) or ciphertext (when decrypting), one 32-bit word at a time. To fit different data sizes, the data can be swapped after processing by configuring the DATATYPE bits in the CRYP_CR register. Refer to Section39.3.16: CRYP data registers and data swapping for more details. When CRYP_DIN register is written to the data are pushed into the input FIFO. If CRYPEN = 1, when at least two 32-bit words in the DES/TDES mode have been pushed into the input FIFO (four words in the AES mode), and when at least two words are free in the output FIFO (four words in the AES mode), the CRYP engine starts an encrypting or decrypting process. When CRYP_DIN register is read: If CRYPEN = 0, the FIFO is popped, and then the data present in the Input FIFO are returned, from the oldest one (first reading) to the newest one (last reading). The IFEM flag must be checked before each read operation to make sure that the FIFO is not empty. if CRYPEN = 1, an undefined value is returned. After the CRYP_DIN register has been read once or several times, the FIFO must be flushed by setting the FFLUSH bit prior to processing new data.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cryp_din::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cryp_din::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cryp_din`]
module"]
pub type CRYP_DIN = crate::Reg<cryp_din::CRYP_DINrs>;
#[doc = "The CRYP_DIN register is the data input register. It is 32-bit wide. It is used to enter into the input FIFO up to four 64-bit blocks (TDES) or two 128-bit blocks (AES) of plaintext (when encrypting) or ciphertext (when decrypting), one 32-bit word at a time. To fit different data sizes, the data can be swapped after processing by configuring the DATATYPE bits in the CRYP_CR register. Refer to Section39.3.16: CRYP data registers and data swapping for more details. When CRYP_DIN register is written to the data are pushed into the input FIFO. If CRYPEN = 1, when at least two 32-bit words in the DES/TDES mode have been pushed into the input FIFO (four words in the AES mode), and when at least two words are free in the output FIFO (four words in the AES mode), the CRYP engine starts an encrypting or decrypting process. When CRYP_DIN register is read: If CRYPEN = 0, the FIFO is popped, and then the data present in the Input FIFO are returned, from the oldest one (first reading) to the newest one (last reading). The IFEM flag must be checked before each read operation to make sure that the FIFO is not empty. if CRYPEN = 1, an undefined value is returned. After the CRYP_DIN register has been read once or several times, the FIFO must be flushed by setting the FFLUSH bit prior to processing new data."]
pub mod cryp_din;
#[doc = "CRYP_DOUT (r) register accessor: The CRYP_DOUT register is the data output register. It is read-only and 32-bit wide. It is used to retrieve from the output FIFO up to four 64-bit blocks (TDES) or two 128-bit blocks (AES) of plaintext (when encrypting) or ciphertext (when decrypting), one 32-bit word at a time. To fit different data sizes, the data can be swapped after processing by configuring the DATATYPE bits in the CRYP_CR register. Refer to Section39.3.16: CRYP data registers and data swapping for more details. When CRYP_DOUT register is read, the last data entered into the output FIFO (pointed to by the read pointer) is returned.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cryp_dout::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cryp_dout`]
module"]
pub type CRYP_DOUT = crate::Reg<cryp_dout::CRYP_DOUTrs>;
#[doc = "The CRYP_DOUT register is the data output register. It is read-only and 32-bit wide. It is used to retrieve from the output FIFO up to four 64-bit blocks (TDES) or two 128-bit blocks (AES) of plaintext (when encrypting) or ciphertext (when decrypting), one 32-bit word at a time. To fit different data sizes, the data can be swapped after processing by configuring the DATATYPE bits in the CRYP_CR register. Refer to Section39.3.16: CRYP data registers and data swapping for more details. When CRYP_DOUT register is read, the last data entered into the output FIFO (pointed to by the read pointer) is returned."]
pub mod cryp_dout;
#[doc = "CRYP_DMACR (rw) register accessor: CRYP DMA control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cryp_dmacr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cryp_dmacr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cryp_dmacr`]
module"]
pub type CRYP_DMACR = crate::Reg<cryp_dmacr::CRYP_DMACRrs>;
#[doc = "CRYP DMA control register"]
pub mod cryp_dmacr;
#[doc = "CRYP_IMSCR (rw) register accessor: The CRYP_IMSCR register is the interrupt mask set or clear register. It is a read/write register. When a read operation is performed, this register gives the current value of the mask applied to the relevant interrupt. Writing 1 to the particular bit sets the mask, thus enabling the interrupt to be read. Writing 0 to this bit clears the corresponding mask. All the bits are cleared to 0 when the peripheral is reset.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cryp_imscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cryp_imscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cryp_imscr`]
module"]
pub type CRYP_IMSCR = crate::Reg<cryp_imscr::CRYP_IMSCRrs>;
#[doc = "The CRYP_IMSCR register is the interrupt mask set or clear register. It is a read/write register. When a read operation is performed, this register gives the current value of the mask applied to the relevant interrupt. Writing 1 to the particular bit sets the mask, thus enabling the interrupt to be read. Writing 0 to this bit clears the corresponding mask. All the bits are cleared to 0 when the peripheral is reset."]
pub mod cryp_imscr;
#[doc = "CRYP_RISR (r) register accessor: The CRYP_RISR register is the raw interrupt status register. It is a read-only register. When a read operation is performed, this register gives the current raw status of the corresponding interrupt, i.e. the interrupt information without taking CRYP_IMSCR mask into account. Write operations have no effect.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cryp_risr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cryp_risr`]
module"]
pub type CRYP_RISR = crate::Reg<cryp_risr::CRYP_RISRrs>;
#[doc = "The CRYP_RISR register is the raw interrupt status register. It is a read-only register. When a read operation is performed, this register gives the current raw status of the corresponding interrupt, i.e. the interrupt information without taking CRYP_IMSCR mask into account. Write operations have no effect."]
pub mod cryp_risr;
#[doc = "CRYP_MISR (r) register accessor: The CRYP_MISR register is the masked interrupt status register. It is a read-only register. When a read operation is performed, this register gives the current masked status of the corresponding interrupt, i.e. the interrupt information taking CRYP_IMSCR mask into account. Write operations have no effect.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cryp_misr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cryp_misr`]
module"]
pub type CRYP_MISR = crate::Reg<cryp_misr::CRYP_MISRrs>;
#[doc = "The CRYP_MISR register is the masked interrupt status register. It is a read-only register. When a read operation is performed, this register gives the current masked status of the corresponding interrupt, i.e. the interrupt information taking CRYP_IMSCR mask into account. Write operations have no effect."]
pub mod cryp_misr;
#[doc = "CRYP_K0LR (w) register accessor: CRYP key registers contain the cryptographic keys. In DES/TDES mode, the keys are 64-bit binary values (number from left to right, that is the leftmost bit is bit 1) and named K1, K2 and K3 (K0 is not used). Each key consists of 56 information bits and 8 parity bits. In AES mode, the key is considered as a single 128, 192 or 256 bits long sequence K0K1K2...K127/191/255. The AES key is entered into the registers as follows: for AES-128: K0..K127 corresponds to b127..b0 (b255..b128 are not used), for AES-192: K0..K191 corresponds to b191..b0 (b255..b192 are not used), for AES-256: K0..K255 corresponds to b255..b0. In all cases key bit K0 is the leftmost bit in CRYP inner memory and register bit b0 is the rightmost bit in corresponding CRYP_KxLR key register. For more information refer to Section39.3.17: CRYP key registers. Write accesses to these registers are disregarded when the cryptographic processor is busy (bit BUSY = 1 in the CRYP_SR register)\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cryp_k0lr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cryp_k0lr`]
module"]
pub type CRYP_K0LR = crate::Reg<cryp_k0lr::CRYP_K0LRrs>;
#[doc = "CRYP key registers contain the cryptographic keys. In DES/TDES mode, the keys are 64-bit binary values (number from left to right, that is the leftmost bit is bit 1) and named K1, K2 and K3 (K0 is not used). Each key consists of 56 information bits and 8 parity bits. In AES mode, the key is considered as a single 128, 192 or 256 bits long sequence K0K1K2...K127/191/255. The AES key is entered into the registers as follows: for AES-128: K0..K127 corresponds to b127..b0 (b255..b128 are not used), for AES-192: K0..K191 corresponds to b191..b0 (b255..b192 are not used), for AES-256: K0..K255 corresponds to b255..b0. In all cases key bit K0 is the leftmost bit in CRYP inner memory and register bit b0 is the rightmost bit in corresponding CRYP_KxLR key register. For more information refer to Section39.3.17: CRYP key registers. Write accesses to these registers are disregarded when the cryptographic processor is busy (bit BUSY = 1 in the CRYP_SR register)"]
pub mod cryp_k0lr;
#[doc = "CRYP_K0RR (w) register accessor: Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cryp_k0rr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cryp_k0rr`]
module"]
pub type CRYP_K0RR = crate::Reg<cryp_k0rr::CRYP_K0RRrs>;
#[doc = "Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details."]
pub mod cryp_k0rr;
#[doc = "CRYP_K1LR (w) register accessor: Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cryp_k1lr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cryp_k1lr`]
module"]
pub type CRYP_K1LR = crate::Reg<cryp_k1lr::CRYP_K1LRrs>;
#[doc = "Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details."]
pub mod cryp_k1lr;
#[doc = "CRYP_K1RR (w) register accessor: Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cryp_k1rr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cryp_k1rr`]
module"]
pub type CRYP_K1RR = crate::Reg<cryp_k1rr::CRYP_K1RRrs>;
#[doc = "Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details."]
pub mod cryp_k1rr;
#[doc = "CRYP_K2LR (w) register accessor: Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cryp_k2lr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cryp_k2lr`]
module"]
pub type CRYP_K2LR = crate::Reg<cryp_k2lr::CRYP_K2LRrs>;
#[doc = "Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details."]
pub mod cryp_k2lr;
#[doc = "CRYP_K2RR (w) register accessor: Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cryp_k2rr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cryp_k2rr`]
module"]
pub type CRYP_K2RR = crate::Reg<cryp_k2rr::CRYP_K2RRrs>;
#[doc = "Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details."]
pub mod cryp_k2rr;
#[doc = "CRYP_K3LR (w) register accessor: Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cryp_k3lr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cryp_k3lr`]
module"]
pub type CRYP_K3LR = crate::Reg<cryp_k3lr::CRYP_K3LRrs>;
#[doc = "Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details."]
pub mod cryp_k3lr;
#[doc = "CRYP_K3RR (w) register accessor: Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cryp_k3rr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cryp_k3rr`]
module"]
pub type CRYP_K3RR = crate::Reg<cryp_k3rr::CRYP_K3RRrs>;
#[doc = "Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details."]
pub mod cryp_k3rr;
#[doc = "CRYP_IV0LR (rw) register accessor: The CRYP_IV0...1(L/R)R are the left-word and right-word registers for the initialization vector (64 bits for DES/TDES and 128 bits for AES). For more information refer to Section39.3.18: CRYP initialization vector registers. IV0 is the leftmost bit whereas IV63 (DES, TDES) or IV127 (AES) are the rightmost bits of the initialization vector. IV1(L/R)R is used only in the AES. Only CRYP_IV0(L/R) is used in DES/TDES. Write access to these registers are disregarded when the cryptographic processor is busy (bit BUSY = 1 in the CRYP_SR register).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cryp_iv0lr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cryp_iv0lr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cryp_iv0lr`]
module"]
pub type CRYP_IV0LR = crate::Reg<cryp_iv0lr::CRYP_IV0LRrs>;
#[doc = "The CRYP_IV0...1(L/R)R are the left-word and right-word registers for the initialization vector (64 bits for DES/TDES and 128 bits for AES). For more information refer to Section39.3.18: CRYP initialization vector registers. IV0 is the leftmost bit whereas IV63 (DES, TDES) or IV127 (AES) are the rightmost bits of the initialization vector. IV1(L/R)R is used only in the AES. Only CRYP_IV0(L/R) is used in DES/TDES. Write access to these registers are disregarded when the cryptographic processor is busy (bit BUSY = 1 in the CRYP_SR register)."]
pub mod cryp_iv0lr;
#[doc = "CRYP_IV0RR (rw) register accessor: Refer to Section39.6.17: CRYP initialization vector register 0L (CRYP_IV0LR) for details.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cryp_iv0rr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cryp_iv0rr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cryp_iv0rr`]
module"]
pub type CRYP_IV0RR = crate::Reg<cryp_iv0rr::CRYP_IV0RRrs>;
#[doc = "Refer to Section39.6.17: CRYP initialization vector register 0L (CRYP_IV0LR) for details."]
pub mod cryp_iv0rr;
#[doc = "CRYP_IV1LR (rw) register accessor: Refer to Section39.6.17: CRYP initialization vector register 0L (CRYP_IV0LR) for details.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cryp_iv1lr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cryp_iv1lr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cryp_iv1lr`]
module"]
pub type CRYP_IV1LR = crate::Reg<cryp_iv1lr::CRYP_IV1LRrs>;
#[doc = "Refer to Section39.6.17: CRYP initialization vector register 0L (CRYP_IV0LR) for details."]
pub mod cryp_iv1lr;
#[doc = "CRYP_IV1RR (rw) register accessor: Refer to Section39.6.17: CRYP initialization vector register 0L (CRYP_IV0LR) for details.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cryp_iv1rr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cryp_iv1rr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cryp_iv1rr`]
module"]
pub type CRYP_IV1RR = crate::Reg<cryp_iv1rr::CRYP_IV1RRrs>;
#[doc = "Refer to Section39.6.17: CRYP initialization vector register 0L (CRYP_IV0LR) for details."]
pub mod cryp_iv1rr;
#[doc = "CRYP_CSGCMCCM0R (rw) register accessor: These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cryp_csgcmccm0r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cryp_csgcmccm0r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cryp_csgcmccm0r`]
module"]
pub type CRYP_CSGCMCCM0R = crate::Reg<cryp_csgcmccm0r::CRYP_CSGCMCCM0Rrs>;
#[doc = "These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers."]
pub mod cryp_csgcmccm0r;
#[doc = "CRYP_CSGCMCCM1R (rw) register accessor: These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cryp_csgcmccm1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cryp_csgcmccm1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cryp_csgcmccm1r`]
module"]
pub type CRYP_CSGCMCCM1R = crate::Reg<cryp_csgcmccm1r::CRYP_CSGCMCCM1Rrs>;
#[doc = "These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers."]
pub mod cryp_csgcmccm1r;
#[doc = "CRYP_CSGCMCCM2R (rw) register accessor: These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cryp_csgcmccm2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cryp_csgcmccm2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cryp_csgcmccm2r`]
module"]
pub type CRYP_CSGCMCCM2R = crate::Reg<cryp_csgcmccm2r::CRYP_CSGCMCCM2Rrs>;
#[doc = "These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers."]
pub mod cryp_csgcmccm2r;
#[doc = "CRYP_CSGCMCCM3R (rw) register accessor: These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cryp_csgcmccm3r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cryp_csgcmccm3r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cryp_csgcmccm3r`]
module"]
pub type CRYP_CSGCMCCM3R = crate::Reg<cryp_csgcmccm3r::CRYP_CSGCMCCM3Rrs>;
#[doc = "These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers."]
pub mod cryp_csgcmccm3r;
#[doc = "CRYP_CSGCMCCM4R (rw) register accessor: These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cryp_csgcmccm4r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cryp_csgcmccm4r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cryp_csgcmccm4r`]
module"]
pub type CRYP_CSGCMCCM4R = crate::Reg<cryp_csgcmccm4r::CRYP_CSGCMCCM4Rrs>;
#[doc = "These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers."]
pub mod cryp_csgcmccm4r;
#[doc = "CRYP_CSGCMCCM5R (rw) register accessor: These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cryp_csgcmccm5r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cryp_csgcmccm5r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cryp_csgcmccm5r`]
module"]
pub type CRYP_CSGCMCCM5R = crate::Reg<cryp_csgcmccm5r::CRYP_CSGCMCCM5Rrs>;
#[doc = "These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers."]
pub mod cryp_csgcmccm5r;
#[doc = "CRYP_CSGCMCCM6R (rw) register accessor: These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cryp_csgcmccm6r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cryp_csgcmccm6r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cryp_csgcmccm6r`]
module"]
pub type CRYP_CSGCMCCM6R = crate::Reg<cryp_csgcmccm6r::CRYP_CSGCMCCM6Rrs>;
#[doc = "These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers."]
pub mod cryp_csgcmccm6r;
#[doc = "CRYP_CSGCMCCM7R (rw) register accessor: These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cryp_csgcmccm7r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cryp_csgcmccm7r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cryp_csgcmccm7r`]
module"]
pub type CRYP_CSGCMCCM7R = crate::Reg<cryp_csgcmccm7r::CRYP_CSGCMCCM7Rrs>;
#[doc = "These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers."]
pub mod cryp_csgcmccm7r;
#[doc = "CRYP_CSGCM0R (rw) register accessor: Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cryp_csgcm0r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cryp_csgcm0r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cryp_csgcm0r`]
module"]
pub type CRYP_CSGCM0R = crate::Reg<cryp_csgcm0r::CRYP_CSGCM0Rrs>;
#[doc = "Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details."]
pub mod cryp_csgcm0r;
#[doc = "CRYP_CSGCM1R (rw) register accessor: Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cryp_csgcm1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cryp_csgcm1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cryp_csgcm1r`]
module"]
pub type CRYP_CSGCM1R = crate::Reg<cryp_csgcm1r::CRYP_CSGCM1Rrs>;
#[doc = "Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details."]
pub mod cryp_csgcm1r;
#[doc = "CRYP_CSGCM2R (rw) register accessor: Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cryp_csgcm2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cryp_csgcm2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cryp_csgcm2r`]
module"]
pub type CRYP_CSGCM2R = crate::Reg<cryp_csgcm2r::CRYP_CSGCM2Rrs>;
#[doc = "Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details."]
pub mod cryp_csgcm2r;
#[doc = "CRYP_CSGCM3R (rw) register accessor: Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cryp_csgcm3r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cryp_csgcm3r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cryp_csgcm3r`]
module"]
pub type CRYP_CSGCM3R = crate::Reg<cryp_csgcm3r::CRYP_CSGCM3Rrs>;
#[doc = "Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details."]
pub mod cryp_csgcm3r;
#[doc = "CRYP_CSGCM4R (rw) register accessor: Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cryp_csgcm4r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cryp_csgcm4r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cryp_csgcm4r`]
module"]
pub type CRYP_CSGCM4R = crate::Reg<cryp_csgcm4r::CRYP_CSGCM4Rrs>;
#[doc = "Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details."]
pub mod cryp_csgcm4r;
#[doc = "CRYP_CSGCM5R (rw) register accessor: Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cryp_csgcm5r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cryp_csgcm5r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cryp_csgcm5r`]
module"]
pub type CRYP_CSGCM5R = crate::Reg<cryp_csgcm5r::CRYP_CSGCM5Rrs>;
#[doc = "Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details."]
pub mod cryp_csgcm5r;
#[doc = "CRYP_CSGCM6R (rw) register accessor: Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cryp_csgcm6r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cryp_csgcm6r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cryp_csgcm6r`]
module"]
pub type CRYP_CSGCM6R = crate::Reg<cryp_csgcm6r::CRYP_CSGCM6Rrs>;
#[doc = "Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details."]
pub mod cryp_csgcm6r;
#[doc = "CRYP_CSGCM7R (rw) register accessor: Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cryp_csgcm7r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cryp_csgcm7r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cryp_csgcm7r`]
module"]
pub type CRYP_CSGCM7R = crate::Reg<cryp_csgcm7r::CRYP_CSGCM7Rrs>;
#[doc = "Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details."]
pub mod cryp_csgcm7r;
#[doc = "CRYP_HWCFGR (r) register accessor: CRYP hardware configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cryp_hwcfgr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cryp_hwcfgr`]
module"]
pub type CRYP_HWCFGR = crate::Reg<cryp_hwcfgr::CRYP_HWCFGRrs>;
#[doc = "CRYP hardware configuration register"]
pub mod cryp_hwcfgr;
#[doc = "CRYP_VERR (r) register accessor: CRYP HW Version Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cryp_verr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cryp_verr`]
module"]
pub type CRYP_VERR = crate::Reg<cryp_verr::CRYP_VERRrs>;
#[doc = "CRYP HW Version Register"]
pub mod cryp_verr;
#[doc = "CRYP_IPIDR (r) register accessor: CRYP Identification\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cryp_ipidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cryp_ipidr`]
module"]
pub type CRYP_IPIDR = crate::Reg<cryp_ipidr::CRYP_IPIDRrs>;
#[doc = "CRYP Identification"]
pub mod cryp_ipidr;
#[doc = "CRYP_MID (r) register accessor: CRYP HW Magic ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cryp_mid::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cryp_mid`]
module"]
pub type CRYP_MID = crate::Reg<cryp_mid::CRYP_MIDrs>;
#[doc = "CRYP HW Magic ID"]
pub mod cryp_mid;
