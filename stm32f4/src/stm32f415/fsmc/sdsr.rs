///Register `SDSR` reader
pub type R = crate::R<SDSRrs>;
///Field `RE` reader - Refresh error flag
pub type RE_R = crate::BitReader;
///Field `MODES1` reader - Status Mode for Bank 1
pub type MODES1_R = crate::FieldReader;
///Field `MODES2` reader - Status Mode for Bank 2
pub type MODES2_R = crate::FieldReader;
///Field `BUSY` reader - Busy status
pub type BUSY_R = crate::BitReader;
impl R {
    ///Bit 0 - Refresh error flag
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - Status Mode for Bank 1
    #[inline(always)]
    pub fn modes1(&self) -> MODES1_R {
        MODES1_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bits 3:4 - Status Mode for Bank 2
    #[inline(always)]
    pub fn modes2(&self) -> MODES2_R {
        MODES2_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bit 5 - Busy status
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDSR")
            .field("re", &self.re())
            .field("modes1", &self.modes1())
            .field("modes2", &self.modes2())
            .field("busy", &self.busy())
            .finish()
    }
}
/**SDRAM Status register

You can [`read`](crate::Reg::read) this register and get [`sdsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#FSMC:SDSR)*/
pub struct SDSRrs;
impl crate::RegisterSpec for SDSRrs {
    type Ux = u32;
}
///`read()` method returns [`sdsr::R`](R) reader structure
impl crate::Readable for SDSRrs {}
///`reset()` method sets SDSR to value 0
impl crate::Resettable for SDSRrs {}
