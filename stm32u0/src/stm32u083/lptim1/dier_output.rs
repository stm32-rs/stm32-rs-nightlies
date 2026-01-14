///Register `DIER_OUTPUT` reader
pub type R = crate::R<DIER_OUTPUTrs>;
///Register `DIER_OUTPUT` writer
pub type W = crate::W<DIER_OUTPUTrs>;
///Field `CC1IE` reader - Capture/compare 1 interrupt enable
pub type CC1IE_R = crate::BitReader;
///Field `CC1IE` writer - Capture/compare 1 interrupt enable
pub type CC1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ARRMIE` reader - Autoreload match Interrupt Enable
pub type ARRMIE_R = crate::BitReader;
///Field `ARRMIE` writer - Autoreload match Interrupt Enable
pub type ARRMIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EXTTRIGIE` reader - External trigger valid edge Interrupt Enable
pub type EXTTRIGIE_R = crate::BitReader;
///Field `EXTTRIGIE` writer - External trigger valid edge Interrupt Enable
pub type EXTTRIGIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMP1OKIE` reader - Compare register 1 update OK interrupt enable
pub type CMP1OKIE_R = crate::BitReader;
///Field `CMP1OKIE` writer - Compare register 1 update OK interrupt enable
pub type CMP1OKIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ARROKIE` reader - Autoreload register update OK Interrupt Enable
pub type ARROKIE_R = crate::BitReader;
///Field `ARROKIE` writer - Autoreload register update OK Interrupt Enable
pub type ARROKIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UPIE` reader - Direction change to UP Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section125.3.
pub type UPIE_R = crate::BitReader;
///Field `UPIE` writer - Direction change to UP Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section125.3.
pub type UPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DOWNIE` reader - Direction change to down Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section125.3.
pub type DOWNIE_R = crate::BitReader;
///Field `DOWNIE` writer - Direction change to down Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section125.3.
pub type DOWNIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UEIE` reader - Update event interrupt enable
pub type UEIE_R = crate::BitReader;
///Field `UEIE` writer - Update event interrupt enable
pub type UEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REPOKIE` reader - Repetition register update OK interrupt Enable
pub type REPOKIE_R = crate::BitReader;
///Field `REPOKIE` writer - Repetition register update OK interrupt Enable
pub type REPOKIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2IE` reader - Capture/compare 2 interrupt enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section125.3.
pub type CC2IE_R = crate::BitReader;
///Field `CC2IE` writer - Capture/compare 2 interrupt enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section125.3.
pub type CC2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC3IE` reader - Capture/compare 3 interrupt enable Note: If LPTIM does not implement at least 3 channels this bit is reserved. Refer to Section125.3.
pub type CC3IE_R = crate::BitReader;
///Field `CC3IE` writer - Capture/compare 3 interrupt enable Note: If LPTIM does not implement at least 3 channels this bit is reserved. Refer to Section125.3.
pub type CC3IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC4IE` reader - Capture/compare 4 interrupt enable Note: If LPTIM does not implement at least 4 channels this bit is reserved. Refer to Section125.3.
pub type CC4IE_R = crate::BitReader;
///Field `CC4IE` writer - Capture/compare 4 interrupt enable Note: If LPTIM does not implement at least 4 channels this bit is reserved. Refer to Section125.3.
pub type CC4IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMP2OKIE` reader - Compare register 2 update OK interrupt enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section125.3.
pub type CMP2OKIE_R = crate::BitReader;
///Field `CMP2OKIE` writer - Compare register 2 update OK interrupt enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section125.3.
pub type CMP2OKIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMP3OKIE` reader - Compare register 3 update OK interrupt enable Note: If LPTIM does not implement at least 3 channels this bit is reserved. Refer to Section125.3.
pub type CMP3OKIE_R = crate::BitReader;
///Field `CMP3OKIE` writer - Compare register 3 update OK interrupt enable Note: If LPTIM does not implement at least 3 channels this bit is reserved. Refer to Section125.3.
pub type CMP3OKIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMP4OKIE` reader - Compare register 4 update OK interrupt enable Note: If LPTIM does not implement at least 4 channels this bit is reserved. Refer to Section125.3.
pub type CMP4OKIE_R = crate::BitReader;
///Field `CMP4OKIE` writer - Compare register 4 update OK interrupt enable Note: If LPTIM does not implement at least 4 channels this bit is reserved. Refer to Section125.3.
pub type CMP4OKIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UEDE` reader - Update event DMA request enable Note: If LPTIM does not implement at least 1 channel this bit is reserved. Refer to Section125.3.
pub type UEDE_R = crate::BitReader;
///Field `UEDE` writer - Update event DMA request enable Note: If LPTIM does not implement at least 1 channel this bit is reserved. Refer to Section125.3.
pub type UEDE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Capture/compare 1 interrupt enable
    #[inline(always)]
    pub fn cc1ie(&self) -> CC1IE_R {
        CC1IE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Autoreload match Interrupt Enable
    #[inline(always)]
    pub fn arrmie(&self) -> ARRMIE_R {
        ARRMIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - External trigger valid edge Interrupt Enable
    #[inline(always)]
    pub fn exttrigie(&self) -> EXTTRIGIE_R {
        EXTTRIGIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Compare register 1 update OK interrupt enable
    #[inline(always)]
    pub fn cmp1okie(&self) -> CMP1OKIE_R {
        CMP1OKIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Autoreload register update OK Interrupt Enable
    #[inline(always)]
    pub fn arrokie(&self) -> ARROKIE_R {
        ARROKIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Direction change to UP Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section125.3.
    #[inline(always)]
    pub fn upie(&self) -> UPIE_R {
        UPIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Direction change to down Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section125.3.
    #[inline(always)]
    pub fn downie(&self) -> DOWNIE_R {
        DOWNIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Update event interrupt enable
    #[inline(always)]
    pub fn ueie(&self) -> UEIE_R {
        UEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Repetition register update OK interrupt Enable
    #[inline(always)]
    pub fn repokie(&self) -> REPOKIE_R {
        REPOKIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Capture/compare 2 interrupt enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section125.3.
    #[inline(always)]
    pub fn cc2ie(&self) -> CC2IE_R {
        CC2IE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Capture/compare 3 interrupt enable Note: If LPTIM does not implement at least 3 channels this bit is reserved. Refer to Section125.3.
    #[inline(always)]
    pub fn cc3ie(&self) -> CC3IE_R {
        CC3IE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Capture/compare 4 interrupt enable Note: If LPTIM does not implement at least 4 channels this bit is reserved. Refer to Section125.3.
    #[inline(always)]
    pub fn cc4ie(&self) -> CC4IE_R {
        CC4IE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 19 - Compare register 2 update OK interrupt enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section125.3.
    #[inline(always)]
    pub fn cmp2okie(&self) -> CMP2OKIE_R {
        CMP2OKIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Compare register 3 update OK interrupt enable Note: If LPTIM does not implement at least 3 channels this bit is reserved. Refer to Section125.3.
    #[inline(always)]
    pub fn cmp3okie(&self) -> CMP3OKIE_R {
        CMP3OKIE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Compare register 4 update OK interrupt enable Note: If LPTIM does not implement at least 4 channels this bit is reserved. Refer to Section125.3.
    #[inline(always)]
    pub fn cmp4okie(&self) -> CMP4OKIE_R {
        CMP4OKIE_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 23 - Update event DMA request enable Note: If LPTIM does not implement at least 1 channel this bit is reserved. Refer to Section125.3.
    #[inline(always)]
    pub fn uede(&self) -> UEDE_R {
        UEDE_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIER_OUTPUT")
            .field("cc1ie", &self.cc1ie())
            .field("arrmie", &self.arrmie())
            .field("exttrigie", &self.exttrigie())
            .field("cmp1okie", &self.cmp1okie())
            .field("arrokie", &self.arrokie())
            .field("upie", &self.upie())
            .field("downie", &self.downie())
            .field("ueie", &self.ueie())
            .field("repokie", &self.repokie())
            .field("cc2ie", &self.cc2ie())
            .field("cc3ie", &self.cc3ie())
            .field("cc4ie", &self.cc4ie())
            .field("cmp2okie", &self.cmp2okie())
            .field("cmp3okie", &self.cmp3okie())
            .field("cmp4okie", &self.cmp4okie())
            .field("uede", &self.uede())
            .finish()
    }
}
impl W {
    ///Bit 0 - Capture/compare 1 interrupt enable
    #[inline(always)]
    pub fn cc1ie(&mut self) -> CC1IE_W<'_, DIER_OUTPUTrs> {
        CC1IE_W::new(self, 0)
    }
    ///Bit 1 - Autoreload match Interrupt Enable
    #[inline(always)]
    pub fn arrmie(&mut self) -> ARRMIE_W<'_, DIER_OUTPUTrs> {
        ARRMIE_W::new(self, 1)
    }
    ///Bit 2 - External trigger valid edge Interrupt Enable
    #[inline(always)]
    pub fn exttrigie(&mut self) -> EXTTRIGIE_W<'_, DIER_OUTPUTrs> {
        EXTTRIGIE_W::new(self, 2)
    }
    ///Bit 3 - Compare register 1 update OK interrupt enable
    #[inline(always)]
    pub fn cmp1okie(&mut self) -> CMP1OKIE_W<'_, DIER_OUTPUTrs> {
        CMP1OKIE_W::new(self, 3)
    }
    ///Bit 4 - Autoreload register update OK Interrupt Enable
    #[inline(always)]
    pub fn arrokie(&mut self) -> ARROKIE_W<'_, DIER_OUTPUTrs> {
        ARROKIE_W::new(self, 4)
    }
    ///Bit 5 - Direction change to UP Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section125.3.
    #[inline(always)]
    pub fn upie(&mut self) -> UPIE_W<'_, DIER_OUTPUTrs> {
        UPIE_W::new(self, 5)
    }
    ///Bit 6 - Direction change to down Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section125.3.
    #[inline(always)]
    pub fn downie(&mut self) -> DOWNIE_W<'_, DIER_OUTPUTrs> {
        DOWNIE_W::new(self, 6)
    }
    ///Bit 7 - Update event interrupt enable
    #[inline(always)]
    pub fn ueie(&mut self) -> UEIE_W<'_, DIER_OUTPUTrs> {
        UEIE_W::new(self, 7)
    }
    ///Bit 8 - Repetition register update OK interrupt Enable
    #[inline(always)]
    pub fn repokie(&mut self) -> REPOKIE_W<'_, DIER_OUTPUTrs> {
        REPOKIE_W::new(self, 8)
    }
    ///Bit 9 - Capture/compare 2 interrupt enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section125.3.
    #[inline(always)]
    pub fn cc2ie(&mut self) -> CC2IE_W<'_, DIER_OUTPUTrs> {
        CC2IE_W::new(self, 9)
    }
    ///Bit 10 - Capture/compare 3 interrupt enable Note: If LPTIM does not implement at least 3 channels this bit is reserved. Refer to Section125.3.
    #[inline(always)]
    pub fn cc3ie(&mut self) -> CC3IE_W<'_, DIER_OUTPUTrs> {
        CC3IE_W::new(self, 10)
    }
    ///Bit 11 - Capture/compare 4 interrupt enable Note: If LPTIM does not implement at least 4 channels this bit is reserved. Refer to Section125.3.
    #[inline(always)]
    pub fn cc4ie(&mut self) -> CC4IE_W<'_, DIER_OUTPUTrs> {
        CC4IE_W::new(self, 11)
    }
    ///Bit 19 - Compare register 2 update OK interrupt enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section125.3.
    #[inline(always)]
    pub fn cmp2okie(&mut self) -> CMP2OKIE_W<'_, DIER_OUTPUTrs> {
        CMP2OKIE_W::new(self, 19)
    }
    ///Bit 20 - Compare register 3 update OK interrupt enable Note: If LPTIM does not implement at least 3 channels this bit is reserved. Refer to Section125.3.
    #[inline(always)]
    pub fn cmp3okie(&mut self) -> CMP3OKIE_W<'_, DIER_OUTPUTrs> {
        CMP3OKIE_W::new(self, 20)
    }
    ///Bit 21 - Compare register 4 update OK interrupt enable Note: If LPTIM does not implement at least 4 channels this bit is reserved. Refer to Section125.3.
    #[inline(always)]
    pub fn cmp4okie(&mut self) -> CMP4OKIE_W<'_, DIER_OUTPUTrs> {
        CMP4OKIE_W::new(self, 21)
    }
    ///Bit 23 - Update event DMA request enable Note: If LPTIM does not implement at least 1 channel this bit is reserved. Refer to Section125.3.
    #[inline(always)]
    pub fn uede(&mut self) -> UEDE_W<'_, DIER_OUTPUTrs> {
        UEDE_W::new(self, 23)
    }
}
/**LPTIM1 interrupt enable register \[alternate\]

You can [`read`](crate::Reg::read) this register and get [`dier_output::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dier_output::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#LPTIM1:DIER_OUTPUT)*/
pub struct DIER_OUTPUTrs;
impl crate::RegisterSpec for DIER_OUTPUTrs {
    type Ux = u32;
}
///`read()` method returns [`dier_output::R`](R) reader structure
impl crate::Readable for DIER_OUTPUTrs {}
///`write(|w| ..)` method takes [`dier_output::W`](W) writer structure
impl crate::Writable for DIER_OUTPUTrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DIER_OUTPUT to value 0
impl crate::Resettable for DIER_OUTPUTrs {}
