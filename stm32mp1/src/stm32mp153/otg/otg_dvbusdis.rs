#[doc = "Register `OTG_DVBUSDIS` reader"]
pub type R = crate::R<OTG_DVBUSDISrs>;
#[doc = "Register `OTG_DVBUSDIS` writer"]
pub type W = crate::W<OTG_DVBUSDISrs>;
#[doc = "Field `VBUSDT` reader - VBUSDT"]
pub type VBUSDT_R = crate::FieldReader<u16>;
#[doc = "Field `VBUSDT` writer - VBUSDT"]
pub type VBUSDT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - VBUSDT"]
    #[inline(always)]
    pub fn vbusdt(&self) -> VBUSDT_R {
        VBUSDT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - VBUSDT"]
    #[inline(always)]
    #[must_use]
    pub fn vbusdt(&mut self) -> VBUSDT_W<OTG_DVBUSDISrs> {
        VBUSDT_W::new(self, 0)
    }
}
#[doc = "This register specifies the VBUS discharge time after VBUS pulsing during SRP.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_dvbusdis::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_dvbusdis::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTG_DVBUSDISrs;
impl crate::RegisterSpec for OTG_DVBUSDISrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_dvbusdis::R`](R) reader structure"]
impl crate::Readable for OTG_DVBUSDISrs {}
#[doc = "`write(|w| ..)` method takes [`otg_dvbusdis::W`](W) writer structure"]
impl crate::Writable for OTG_DVBUSDISrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OTG_DVBUSDIS to value 0x17d7"]
impl crate::Resettable for OTG_DVBUSDISrs {
    const RESET_VALUE: u32 = 0x17d7;
}
