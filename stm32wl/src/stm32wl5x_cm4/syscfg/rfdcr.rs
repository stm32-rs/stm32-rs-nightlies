#[doc = "Register `RFDCR` reader"]
pub type R = crate::R<RFDCRrs>;
#[doc = "Register `RFDCR` writer"]
pub type W = crate::W<RFDCRrs>;
#[doc = "radio debug test bus selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFTBSEL {
    #[doc = "0: Digital test bus selected on RF_ADTB\\[3:0\\]"]
    Digital = 0,
    #[doc = "1: Analog test bus selected on RF_ADTB\\[3:0\\]"]
    Analog = 1,
}
impl From<RFTBSEL> for bool {
    #[inline(always)]
    fn from(variant: RFTBSEL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFTBSEL` reader - radio debug test bus selection"]
pub type RFTBSEL_R = crate::BitReader<RFTBSEL>;
impl RFTBSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RFTBSEL {
        match self.bits {
            false => RFTBSEL::Digital,
            true => RFTBSEL::Analog,
        }
    }
    #[doc = "Digital test bus selected on RF_ADTB\\[3:0\\]"]
    #[inline(always)]
    pub fn is_digital(&self) -> bool {
        *self == RFTBSEL::Digital
    }
    #[doc = "Analog test bus selected on RF_ADTB\\[3:0\\]"]
    #[inline(always)]
    pub fn is_analog(&self) -> bool {
        *self == RFTBSEL::Analog
    }
}
#[doc = "Field `RFTBSEL` writer - radio debug test bus selection"]
pub type RFTBSEL_W<'a, REG> = crate::BitWriter<'a, REG, RFTBSEL>;
impl<'a, REG> RFTBSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Digital test bus selected on RF_ADTB\\[3:0\\]"]
    #[inline(always)]
    pub fn digital(self) -> &'a mut crate::W<REG> {
        self.variant(RFTBSEL::Digital)
    }
    #[doc = "Analog test bus selected on RF_ADTB\\[3:0\\]"]
    #[inline(always)]
    pub fn analog(self) -> &'a mut crate::W<REG> {
        self.variant(RFTBSEL::Analog)
    }
}
impl R {
    #[doc = "Bit 0 - radio debug test bus selection"]
    #[inline(always)]
    pub fn rftbsel(&self) -> RFTBSEL_R {
        RFTBSEL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - radio debug test bus selection"]
    #[inline(always)]
    #[must_use]
    pub fn rftbsel(&mut self) -> RFTBSEL_W<RFDCRrs> {
        RFTBSEL_W::new(self, 0)
    }
}
#[doc = "radio debug control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfdcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfdcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RFDCRrs;
impl crate::RegisterSpec for RFDCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfdcr::R`](R) reader structure"]
impl crate::Readable for RFDCRrs {}
#[doc = "`write(|w| ..)` method takes [`rfdcr::W`](W) writer structure"]
impl crate::Writable for RFDCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RFDCR to value 0"]
impl crate::Resettable for RFDCRrs {
    const RESET_VALUE: u32 = 0;
}
