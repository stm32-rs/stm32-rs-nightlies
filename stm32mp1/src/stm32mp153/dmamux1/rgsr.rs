///Register `RGSR` reader
pub type R = crate::R<RGSRrs>;
///Field `OF0` reader - OF0
pub type OF0_R = crate::BitReader;
///Field `OF1` reader - OF1
pub type OF1_R = crate::BitReader;
///Field `OF2` reader - OF2
pub type OF2_R = crate::BitReader;
///Field `OF3` reader - OF3
pub type OF3_R = crate::BitReader;
///Field `OF4` reader - OF4
pub type OF4_R = crate::BitReader;
///Field `OF5` reader - OF5
pub type OF5_R = crate::BitReader;
///Field `OF6` reader - OF6
pub type OF6_R = crate::BitReader;
///Field `OF7` reader - OF7
pub type OF7_R = crate::BitReader;
impl R {
    ///Bit 0 - OF0
    #[inline(always)]
    pub fn of0(&self) -> OF0_R {
        OF0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - OF1
    #[inline(always)]
    pub fn of1(&self) -> OF1_R {
        OF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - OF2
    #[inline(always)]
    pub fn of2(&self) -> OF2_R {
        OF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - OF3
    #[inline(always)]
    pub fn of3(&self) -> OF3_R {
        OF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - OF4
    #[inline(always)]
    pub fn of4(&self) -> OF4_R {
        OF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - OF5
    #[inline(always)]
    pub fn of5(&self) -> OF5_R {
        OF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - OF6
    #[inline(always)]
    pub fn of6(&self) -> OF6_R {
        OF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - OF7
    #[inline(always)]
    pub fn of7(&self) -> OF7_R {
        OF7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RGSR")
            .field("of0", &self.of0())
            .field("of1", &self.of1())
            .field("of2", &self.of2())
            .field("of3", &self.of3())
            .field("of4", &self.of4())
            .field("of5", &self.of5())
            .field("of6", &self.of6())
            .field("of7", &self.of7())
            .finish()
    }
}
/**DMAMUX request generator interrupt status register

You can [`read`](crate::Reg::read) this register and get [`rgsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DMAMUX1:RGSR)*/
pub struct RGSRrs;
impl crate::RegisterSpec for RGSRrs {
    type Ux = u32;
}
///`read()` method returns [`rgsr::R`](R) reader structure
impl crate::Readable for RGSRrs {}
///`reset()` method sets RGSR to value 0
impl crate::Resettable for RGSRrs {}
