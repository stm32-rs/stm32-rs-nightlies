///Register `RGSR` reader
pub type R = crate::R<RGSRrs>;
///Field `OF0` reader - Generator Overrun Flag 0
pub type OF0_R = crate::BitReader;
///Field `OF1` reader - Generator Overrun Flag 1
pub type OF1_R = crate::BitReader;
///Field `OF2` reader - Generator Overrun Flag 2
pub type OF2_R = crate::BitReader;
///Field `OF3` reader - Generator Overrun Flag 3
pub type OF3_R = crate::BitReader;
impl R {
    ///Bit 0 - Generator Overrun Flag 0
    #[inline(always)]
    pub fn of0(&self) -> OF0_R {
        OF0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Generator Overrun Flag 1
    #[inline(always)]
    pub fn of1(&self) -> OF1_R {
        OF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Generator Overrun Flag 2
    #[inline(always)]
    pub fn of2(&self) -> OF2_R {
        OF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Generator Overrun Flag 3
    #[inline(always)]
    pub fn of3(&self) -> OF3_R {
        OF3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RGSR")
            .field("of0", &self.of0())
            .field("of1", &self.of1())
            .field("of2", &self.of2())
            .field("of3", &self.of3())
            .finish()
    }
}
/**DMA Request Generator Status Register

You can [`read`](crate::Reg::read) this register and get [`rgsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#DMAMUX1:RGSR)*/
pub struct RGSRrs;
impl crate::RegisterSpec for RGSRrs {
    type Ux = u32;
}
///`read()` method returns [`rgsr::R`](R) reader structure
impl crate::Readable for RGSRrs {}
///`reset()` method sets RGSR to value 0
impl crate::Resettable for RGSRrs {
    const RESET_VALUE: u32 = 0;
}
