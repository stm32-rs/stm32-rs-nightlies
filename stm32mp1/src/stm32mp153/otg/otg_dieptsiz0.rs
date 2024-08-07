///Register `OTG_DIEPTSIZ0` reader
pub type R = crate::R<OTG_DIEPTSIZ0rs>;
///Register `OTG_DIEPTSIZ0` writer
pub type W = crate::W<OTG_DIEPTSIZ0rs>;
///Field `XFRSIZ` reader - XFRSIZ
pub type XFRSIZ_R = crate::FieldReader;
///Field `XFRSIZ` writer - XFRSIZ
pub type XFRSIZ_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `PKTCNT` reader - PKTCNT
pub type PKTCNT_R = crate::FieldReader;
///Field `PKTCNT` writer - PKTCNT
pub type PKTCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:6 - XFRSIZ
    #[inline(always)]
    pub fn xfrsiz(&self) -> XFRSIZ_R {
        XFRSIZ_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 19:20 - PKTCNT
    #[inline(always)]
    pub fn pktcnt(&self) -> PKTCNT_R {
        PKTCNT_R::new(((self.bits >> 19) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTG_DIEPTSIZ0")
            .field("xfrsiz", &self.xfrsiz())
            .field("pktcnt", &self.pktcnt())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - XFRSIZ
    #[inline(always)]
    #[must_use]
    pub fn xfrsiz(&mut self) -> XFRSIZ_W<OTG_DIEPTSIZ0rs> {
        XFRSIZ_W::new(self, 0)
    }
    ///Bits 19:20 - PKTCNT
    #[inline(always)]
    #[must_use]
    pub fn pktcnt(&mut self) -> PKTCNT_W<OTG_DIEPTSIZ0rs> {
        PKTCNT_W::new(self, 19)
    }
}
/**The application must modify this register before enabling endpoint 0.

You can [`read`](crate::Reg::read) this register and get [`otg_dieptsiz0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_dieptsiz0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#OTG:OTG_DIEPTSIZ0)*/
pub struct OTG_DIEPTSIZ0rs;
impl crate::RegisterSpec for OTG_DIEPTSIZ0rs {
    type Ux = u32;
}
///`read()` method returns [`otg_dieptsiz0::R`](R) reader structure
impl crate::Readable for OTG_DIEPTSIZ0rs {}
///`write(|w| ..)` method takes [`otg_dieptsiz0::W`](W) writer structure
impl crate::Writable for OTG_DIEPTSIZ0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets OTG_DIEPTSIZ0 to value 0
impl crate::Resettable for OTG_DIEPTSIZ0rs {
    const RESET_VALUE: u32 = 0;
}
