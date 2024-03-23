#[doc = "Register `EGR` writer"]
pub type W = crate::W<EGRrs>;
#[doc = "Update generation This bit can be set by software, it is automatically cleared by hardware.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UG {
    #[doc = "1: Re-initializes the timer counter and generates an update of the registers."]
    Update = 1,
}
impl From<UG> for bool {
    #[inline(always)]
    fn from(variant: UG) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UG` writer - Update generation This bit can be set by software, it is automatically cleared by hardware."]
pub type UG_W<'a, REG> = crate::BitWriter<'a, REG, UG>;
impl<'a, REG> UG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Re-initializes the timer counter and generates an update of the registers."]
    #[inline(always)]
    pub fn update(self) -> &'a mut crate::W<REG> {
        self.variant(UG::Update)
    }
}
#[doc = "Field `CCG(1-4)` writer - Capture/compare %s generation"]
pub type CCG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMG` writer - Capture/Compare control update generation This bit can be set by software, it is automatically cleared by hardware Note: This bit acts only on channels having a complementary output."]
pub type COMG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TG` writer - Trigger generation This bit is set by software in order to generate an event, it is automatically cleared by hardware."]
pub type TG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BG` writer - Break generation This bit is set by software in order to generate an event, it is automatically cleared by hardware."]
pub type BG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `B2G` writer - Break 2 generation This bit is set by software in order to generate an event, it is automatically cleared by hardware."]
pub type B2G_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Update generation This bit can be set by software, it is automatically cleared by hardware."]
    #[inline(always)]
    #[must_use]
    pub fn ug(&mut self) -> UG_W<EGRrs> {
        UG_W::new(self, 0)
    }
    #[doc = "Capture/compare (1-4) generation"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CC1G` field"]
    #[inline(always)]
    #[must_use]
    pub fn ccg(&mut self, n: u8) -> CCG_W<EGRrs> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CCG_W::new(self, n + 1)
    }
    #[doc = "Bit 1 - Capture/compare 1 generation"]
    #[inline(always)]
    #[must_use]
    pub fn cc1g(&mut self) -> CCG_W<EGRrs> {
        CCG_W::new(self, 1)
    }
    #[doc = "Bit 2 - Capture/compare 2 generation"]
    #[inline(always)]
    #[must_use]
    pub fn cc2g(&mut self) -> CCG_W<EGRrs> {
        CCG_W::new(self, 2)
    }
    #[doc = "Bit 3 - Capture/compare 3 generation"]
    #[inline(always)]
    #[must_use]
    pub fn cc3g(&mut self) -> CCG_W<EGRrs> {
        CCG_W::new(self, 3)
    }
    #[doc = "Bit 4 - Capture/compare 4 generation"]
    #[inline(always)]
    #[must_use]
    pub fn cc4g(&mut self) -> CCG_W<EGRrs> {
        CCG_W::new(self, 4)
    }
    #[doc = "Bit 5 - Capture/Compare control update generation This bit can be set by software, it is automatically cleared by hardware Note: This bit acts only on channels having a complementary output."]
    #[inline(always)]
    #[must_use]
    pub fn comg(&mut self) -> COMG_W<EGRrs> {
        COMG_W::new(self, 5)
    }
    #[doc = "Bit 6 - Trigger generation This bit is set by software in order to generate an event, it is automatically cleared by hardware."]
    #[inline(always)]
    #[must_use]
    pub fn tg(&mut self) -> TG_W<EGRrs> {
        TG_W::new(self, 6)
    }
    #[doc = "Bit 7 - Break generation This bit is set by software in order to generate an event, it is automatically cleared by hardware."]
    #[inline(always)]
    #[must_use]
    pub fn bg(&mut self) -> BG_W<EGRrs> {
        BG_W::new(self, 7)
    }
    #[doc = "Bit 8 - Break 2 generation This bit is set by software in order to generate an event, it is automatically cleared by hardware."]
    #[inline(always)]
    #[must_use]
    pub fn b2g(&mut self) -> B2G_W<EGRrs> {
        B2G_W::new(self, 8)
    }
}
#[doc = "event generation register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`egr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EGRrs;
impl crate::RegisterSpec for EGRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`egr::W`](W) writer structure"]
impl crate::Writable for EGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EGR to value 0"]
impl crate::Resettable for EGRrs {
    const RESET_VALUE: u32 = 0;
}
