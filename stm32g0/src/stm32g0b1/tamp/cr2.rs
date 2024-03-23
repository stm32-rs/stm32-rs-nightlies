#[doc = "Register `CR2` reader"]
pub type R = crate::R<CR2rs>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<CR2rs>;
#[doc = "Field `TAMP1NOER` reader - Tamper 1 no erase"]
pub type TAMP1NOER_R = crate::BitReader;
#[doc = "Field `TAMP1NOER` writer - Tamper 1 no erase"]
pub type TAMP1NOER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP2NOER` reader - Tamper 2 no erase"]
pub type TAMP2NOER_R = crate::BitReader;
#[doc = "Field `TAMP2NOER` writer - Tamper 2 no erase"]
pub type TAMP2NOER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP1MSK` reader - Tamper 1 mask The tamper 1 interrupt must not be enabled when TAMP1MSK is set."]
pub type TAMP1MSK_R = crate::BitReader;
#[doc = "Field `TAMP1MSK` writer - Tamper 1 mask The tamper 1 interrupt must not be enabled when TAMP1MSK is set."]
pub type TAMP1MSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP2MSK` reader - Tamper 2 mask The tamper 2 interrupt must not be enabled when TAMP2MSK is set."]
pub type TAMP2MSK_R = crate::BitReader;
#[doc = "Field `TAMP2MSK` writer - Tamper 2 mask The tamper 2 interrupt must not be enabled when TAMP2MSK is set."]
pub type TAMP2MSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP1TRG` reader - Active level for tamper 1 input (active mode disabled) If TAMPFLT = 00 Tamper 1 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 1 input falling edge and low level triggers a tamper detection event."]
pub type TAMP1TRG_R = crate::BitReader;
#[doc = "Field `TAMP1TRG` writer - Active level for tamper 1 input (active mode disabled) If TAMPFLT = 00 Tamper 1 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 1 input falling edge and low level triggers a tamper detection event."]
pub type TAMP1TRG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP2TRG` reader - Active level for tamper 2 input (active mode disabled) If TAMPFLT = 00 Tamper 2 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 2 input falling edge and low level triggers a tamper detection event."]
pub type TAMP2TRG_R = crate::BitReader;
#[doc = "Field `TAMP2TRG` writer - Active level for tamper 2 input (active mode disabled) If TAMPFLT = 00 Tamper 2 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 2 input falling edge and low level triggers a tamper detection event."]
pub type TAMP2TRG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Tamper 1 no erase"]
    #[inline(always)]
    pub fn tamp1noer(&self) -> TAMP1NOER_R {
        TAMP1NOER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tamper 2 no erase"]
    #[inline(always)]
    pub fn tamp2noer(&self) -> TAMP2NOER_R {
        TAMP2NOER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - Tamper 1 mask The tamper 1 interrupt must not be enabled when TAMP1MSK is set."]
    #[inline(always)]
    pub fn tamp1msk(&self) -> TAMP1MSK_R {
        TAMP1MSK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Tamper 2 mask The tamper 2 interrupt must not be enabled when TAMP2MSK is set."]
    #[inline(always)]
    pub fn tamp2msk(&self) -> TAMP2MSK_R {
        TAMP2MSK_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - Active level for tamper 1 input (active mode disabled) If TAMPFLT = 00 Tamper 1 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 1 input falling edge and low level triggers a tamper detection event."]
    #[inline(always)]
    pub fn tamp1trg(&self) -> TAMP1TRG_R {
        TAMP1TRG_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Active level for tamper 2 input (active mode disabled) If TAMPFLT = 00 Tamper 2 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 2 input falling edge and low level triggers a tamper detection event."]
    #[inline(always)]
    pub fn tamp2trg(&self) -> TAMP2TRG_R {
        TAMP2TRG_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tamper 1 no erase"]
    #[inline(always)]
    #[must_use]
    pub fn tamp1noer(&mut self) -> TAMP1NOER_W<CR2rs> {
        TAMP1NOER_W::new(self, 0)
    }
    #[doc = "Bit 1 - Tamper 2 no erase"]
    #[inline(always)]
    #[must_use]
    pub fn tamp2noer(&mut self) -> TAMP2NOER_W<CR2rs> {
        TAMP2NOER_W::new(self, 1)
    }
    #[doc = "Bit 16 - Tamper 1 mask The tamper 1 interrupt must not be enabled when TAMP1MSK is set."]
    #[inline(always)]
    #[must_use]
    pub fn tamp1msk(&mut self) -> TAMP1MSK_W<CR2rs> {
        TAMP1MSK_W::new(self, 16)
    }
    #[doc = "Bit 17 - Tamper 2 mask The tamper 2 interrupt must not be enabled when TAMP2MSK is set."]
    #[inline(always)]
    #[must_use]
    pub fn tamp2msk(&mut self) -> TAMP2MSK_W<CR2rs> {
        TAMP2MSK_W::new(self, 17)
    }
    #[doc = "Bit 24 - Active level for tamper 1 input (active mode disabled) If TAMPFLT = 00 Tamper 1 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 1 input falling edge and low level triggers a tamper detection event."]
    #[inline(always)]
    #[must_use]
    pub fn tamp1trg(&mut self) -> TAMP1TRG_W<CR2rs> {
        TAMP1TRG_W::new(self, 24)
    }
    #[doc = "Bit 25 - Active level for tamper 2 input (active mode disabled) If TAMPFLT = 00 Tamper 2 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 2 input falling edge and low level triggers a tamper detection event."]
    #[inline(always)]
    #[must_use]
    pub fn tamp2trg(&mut self) -> TAMP2TRG_W<CR2rs> {
        TAMP2TRG_W::new(self, 25)
    }
}
#[doc = "TAMP control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for CR2rs {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2rs {
    const RESET_VALUE: u32 = 0;
}
