///Register `HWCFGR2` reader
pub type R = crate::R<HWCFGR2rs>;
///Field `FIFO_SIZE` reader - FIFO_SIZE
pub type FIFO_SIZE_R = crate::FieldReader;
///Field `WRITE_BUFFERABLE` reader - WRITE_BUFFERABLE
pub type WRITE_BUFFERABLE_R = crate::BitReader;
///Field `CHSEL_WIDTH` reader - CHSEL_WIDTH
pub type CHSEL_WIDTH_R = crate::FieldReader;
impl R {
    ///Bits 0:1 - FIFO_SIZE
    #[inline(always)]
    pub fn fifo_size(&self) -> FIFO_SIZE_R {
        FIFO_SIZE_R::new((self.bits & 3) as u8)
    }
    ///Bit 4 - WRITE_BUFFERABLE
    #[inline(always)]
    pub fn write_bufferable(&self) -> WRITE_BUFFERABLE_R {
        WRITE_BUFFERABLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 8:10 - CHSEL_WIDTH
    #[inline(always)]
    pub fn chsel_width(&self) -> CHSEL_WIDTH_R {
        CHSEL_WIDTH_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWCFGR2")
            .field("fifo_size", &self.fifo_size())
            .field("write_bufferable", &self.write_bufferable())
            .field("chsel_width", &self.chsel_width())
            .finish()
    }
}
/**DMA hardware configuration 2register

You can [`read`](crate::Reg::read) this register and get [`hwcfgr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DMA1:HWCFGR2)*/
pub struct HWCFGR2rs;
impl crate::RegisterSpec for HWCFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`hwcfgr2::R`](R) reader structure
impl crate::Readable for HWCFGR2rs {}
///`reset()` method sets HWCFGR2 to value 0x01
impl crate::Resettable for HWCFGR2rs {
    const RESET_VALUE: u32 = 0x01;
}
