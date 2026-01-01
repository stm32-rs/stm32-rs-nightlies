#[repr(C)]
#[derive(Debug)]
///HUFFMIN cluster: 100-bit minimum Huffman value
pub struct HUFFMIN {
    huffmin_0: HUFFMIN_0,
    huffmin_1: HUFFMIN_1,
    huffmin_2: HUFFMIN_2,
    huffmin_3: HUFFMIN_3,
}
impl HUFFMIN {
    ///0x00 - Bits 0-31 of the minimum Huffman value
    #[inline(always)]
    pub const fn huffmin_0(&self) -> &HUFFMIN_0 {
        &self.huffmin_0
    }
    ///0x04 - Bits 32-63 of the minimum Huffman value
    #[inline(always)]
    pub const fn huffmin_1(&self) -> &HUFFMIN_1 {
        &self.huffmin_1
    }
    ///0x08 - Bits 64-95 of the minimum Huffman value
    #[inline(always)]
    pub const fn huffmin_2(&self) -> &HUFFMIN_2 {
        &self.huffmin_2
    }
    ///0x0c - Bits 96-99 of the minimum Huffman value
    #[inline(always)]
    pub const fn huffmin_3(&self) -> &HUFFMIN_3 {
        &self.huffmin_3
    }
}
/**HUFFMIN_0 (rw) register accessor: Bits 0-31 of the minimum Huffman value

You can [`read`](crate::Reg::read) this register and get [`huffmin_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffmin_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@huffmin_0`] module*/
pub type HUFFMIN_0 = crate::Reg<huffmin_0::HUFFMIN_0rs>;
///Bits 0-31 of the minimum Huffman value
pub mod huffmin_0;
/**HUFFMIN_1 (rw) register accessor: Bits 32-63 of the minimum Huffman value

You can [`read`](crate::Reg::read) this register and get [`huffmin_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffmin_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@huffmin_1`] module*/
pub type HUFFMIN_1 = crate::Reg<huffmin_1::HUFFMIN_1rs>;
///Bits 32-63 of the minimum Huffman value
pub mod huffmin_1;
/**HUFFMIN_2 (rw) register accessor: Bits 64-95 of the minimum Huffman value

You can [`read`](crate::Reg::read) this register and get [`huffmin_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffmin_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@huffmin_2`] module*/
pub type HUFFMIN_2 = crate::Reg<huffmin_2::HUFFMIN_2rs>;
///Bits 64-95 of the minimum Huffman value
pub mod huffmin_2;
/**HUFFMIN_3 (rw) register accessor: Bits 96-99 of the minimum Huffman value

You can [`read`](crate::Reg::read) this register and get [`huffmin_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffmin_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@huffmin_3`] module*/
pub type HUFFMIN_3 = crate::Reg<huffmin_3::HUFFMIN_3rs>;
///Bits 96-99 of the minimum Huffman value
pub mod huffmin_3;
