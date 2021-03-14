#[doc = "Reader of register DMA_HWCFGR2"]
pub type R = crate::R<u32, super::DMA_HWCFGR2>;
#[doc = "Reader of field `FIFO_SIZE`"]
pub type FIFO_SIZE_R = crate::R<u8, u8>;
#[doc = "Reader of field `WRITE_BUFFERABLE`"]
pub type WRITE_BUFFERABLE_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHSEL_WIDTH`"]
pub type CHSEL_WIDTH_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:1 - FIFO_SIZE"]
    #[inline(always)]
    pub fn fifo_size(&self) -> FIFO_SIZE_R {
        FIFO_SIZE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 4 - WRITE_BUFFERABLE"]
    #[inline(always)]
    pub fn write_bufferable(&self) -> WRITE_BUFFERABLE_R {
        WRITE_BUFFERABLE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - CHSEL_WIDTH"]
    #[inline(always)]
    pub fn chsel_width(&self) -> CHSEL_WIDTH_R {
        CHSEL_WIDTH_R::new(((self.bits >> 8) & 0x07) as u8)
    }
}
