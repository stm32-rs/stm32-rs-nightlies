///Register `OTG_HS_HAINTMSK` reader
pub type R = crate::R<OTG_HS_HAINTMSKrs>;
///Register `OTG_HS_HAINTMSK` writer
pub type W = crate::W<OTG_HS_HAINTMSKrs>;
///Field `HAINTM` reader - Channel interrupt mask
pub type HAINTM_R = crate::FieldReader<u16>;
///Field `HAINTM` writer - Channel interrupt mask
pub type HAINTM_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Channel interrupt mask
    #[inline(always)]
    pub fn haintm(&self) -> HAINTM_R {
        HAINTM_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTG_HS_HAINTMSK")
            .field("haintm", &self.haintm())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Channel interrupt mask
    #[inline(always)]
    pub fn haintm(&mut self) -> HAINTM_W<'_, OTG_HS_HAINTMSKrs> {
        HAINTM_W::new(self, 0)
    }
}
/**OTG_HS host all channels interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_haintmsk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_haintmsk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#OTG_HS_HOST:OTG_HS_HAINTMSK)*/
pub struct OTG_HS_HAINTMSKrs;
impl crate::RegisterSpec for OTG_HS_HAINTMSKrs {
    type Ux = u32;
}
///`read()` method returns [`otg_hs_haintmsk::R`](R) reader structure
impl crate::Readable for OTG_HS_HAINTMSKrs {}
///`write(|w| ..)` method takes [`otg_hs_haintmsk::W`](W) writer structure
impl crate::Writable for OTG_HS_HAINTMSKrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OTG_HS_HAINTMSK to value 0
impl crate::Resettable for OTG_HS_HAINTMSKrs {}
