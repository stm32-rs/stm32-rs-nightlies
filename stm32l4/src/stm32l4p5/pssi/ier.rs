#[doc = "Register `IER` reader"]
pub type R = crate::R<IERrs>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IERrs>;
#[doc = "Data buffer overrun/underrun interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVR_IE {
    #[doc = "0: No interrupt generation"]
    Disabled = 0,
    #[doc = "1: An interrupt is generated if either an overrun or an underrun error occurred"]
    Enabled = 1,
}
impl From<OVR_IE> for bool {
    #[inline(always)]
    fn from(variant: OVR_IE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVR_IE` reader - Data buffer overrun/underrun interrupt enable"]
pub type OVR_IE_R = crate::BitReader<OVR_IE>;
impl OVR_IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OVR_IE {
        match self.bits {
            false => OVR_IE::Disabled,
            true => OVR_IE::Enabled,
        }
    }
    #[doc = "No interrupt generation"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OVR_IE::Disabled
    }
    #[doc = "An interrupt is generated if either an overrun or an underrun error occurred"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OVR_IE::Enabled
    }
}
#[doc = "Field `OVR_IE` writer - Data buffer overrun/underrun interrupt enable"]
pub type OVR_IE_W<'a, REG> = crate::BitWriter<'a, REG, OVR_IE>;
impl<'a, REG> OVR_IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt generation"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OVR_IE::Disabled)
    }
    #[doc = "An interrupt is generated if either an overrun or an underrun error occurred"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OVR_IE::Enabled)
    }
}
impl R {
    #[doc = "Bit 1 - Data buffer overrun/underrun interrupt enable"]
    #[inline(always)]
    pub fn ovr_ie(&self) -> OVR_IE_R {
        OVR_IE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Data buffer overrun/underrun interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovr_ie(&mut self) -> OVR_IE_W<IERrs> {
        OVR_IE_W::new(self, 1)
    }
}
#[doc = "PSSI interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IERrs;
impl crate::RegisterSpec for IERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IERrs {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IERrs {
    const RESET_VALUE: u32 = 0;
}
