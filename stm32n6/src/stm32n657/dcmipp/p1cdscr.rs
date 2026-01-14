///Register `P1CDSCR` reader
pub type R = crate::R<P1CDSCRrs>;
///Field `HDIV` reader - Current horizontal division factor, from 128 (8x) to 1023 (1x)
pub type HDIV_R = crate::FieldReader<u16>;
///Field `VDIV` reader - Current vertical division factor, from 128 (8x) to 1023 (1x)
pub type VDIV_R = crate::FieldReader<u16>;
///Field `ENABLE` reader - Current value of bit ENABLE
pub type ENABLE_R = crate::BitReader;
impl R {
    ///Bits 0:9 - Current horizontal division factor, from 128 (8x) to 1023 (1x)
    #[inline(always)]
    pub fn hdiv(&self) -> HDIV_R {
        HDIV_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bits 16:25 - Current vertical division factor, from 128 (8x) to 1023 (1x)
    #[inline(always)]
    pub fn vdiv(&self) -> VDIV_R {
        VDIV_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    ///Bit 31 - Current value of bit ENABLE
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1CDSCR")
            .field("hdiv", &self.hdiv())
            .field("vdiv", &self.vdiv())
            .field("enable", &self.enable())
            .finish()
    }
}
/**DCMIPP Pipex current downsize configuration register

You can [`read`](crate::Reg::read) this register and get [`p1cdscr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CDSCR)*/
pub struct P1CDSCRrs;
impl crate::RegisterSpec for P1CDSCRrs {
    type Ux = u32;
}
///`read()` method returns [`p1cdscr::R`](R) reader structure
impl crate::Readable for P1CDSCRrs {}
///`reset()` method sets P1CDSCR to value 0
impl crate::Resettable for P1CDSCRrs {}
