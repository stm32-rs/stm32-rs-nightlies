#[doc = "Register `CR2` reader"]
pub type R = crate::R<CR2rs>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<CR2rs>;
#[doc = "Master mode selection These bits are used to select the information to be sent in master mode to slave timers for synchronization (TRGO). The combination is as follows: Note: The clock of the slave timer or he peripheral receiving the tim_trgo must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MMS {
    #[doc = "0: Use UG bit from TIMx_EGR register"]
    Reset = 0,
    #[doc = "1: Use CNT bit from TIMx_CEN register"]
    Enable = 1,
    #[doc = "2: Use the update event"]
    Update = 2,
}
impl From<MMS> for u8 {
    #[inline(always)]
    fn from(variant: MMS) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MMS {
    type Ux = u8;
}
#[doc = "Field `MMS` reader - Master mode selection These bits are used to select the information to be sent in master mode to slave timers for synchronization (TRGO). The combination is as follows: Note: The clock of the slave timer or he peripheral receiving the tim_trgo must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
pub type MMS_R = crate::FieldReader<MMS>;
impl MMS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MMS> {
        match self.bits {
            0 => Some(MMS::Reset),
            1 => Some(MMS::Enable),
            2 => Some(MMS::Update),
            _ => None,
        }
    }
    #[doc = "Use UG bit from TIMx_EGR register"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == MMS::Reset
    }
    #[doc = "Use CNT bit from TIMx_CEN register"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MMS::Enable
    }
    #[doc = "Use the update event"]
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == MMS::Update
    }
}
#[doc = "Field `MMS` writer - Master mode selection These bits are used to select the information to be sent in master mode to slave timers for synchronization (TRGO). The combination is as follows: Note: The clock of the slave timer or he peripheral receiving the tim_trgo must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
pub type MMS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, MMS>;
impl<'a, REG> MMS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Use UG bit from TIMx_EGR register"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(MMS::Reset)
    }
    #[doc = "Use CNT bit from TIMx_CEN register"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(MMS::Enable)
    }
    #[doc = "Use the update event"]
    #[inline(always)]
    pub fn update(self) -> &'a mut crate::W<REG> {
        self.variant(MMS::Update)
    }
}
impl R {
    #[doc = "Bits 4:6 - Master mode selection These bits are used to select the information to be sent in master mode to slave timers for synchronization (TRGO). The combination is as follows: Note: The clock of the slave timer or he peripheral receiving the tim_trgo must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    pub fn mms(&self) -> MMS_R {
        MMS_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6 - Master mode selection These bits are used to select the information to be sent in master mode to slave timers for synchronization (TRGO). The combination is as follows: Note: The clock of the slave timer or he peripheral receiving the tim_trgo must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    #[must_use]
    pub fn mms(&mut self) -> MMS_W<CR2rs> {
        MMS_W::new(self, 4)
    }
}
#[doc = "TIM6 control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for CR2rs {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2rs {
    const RESET_VALUE: u16 = 0;
}
