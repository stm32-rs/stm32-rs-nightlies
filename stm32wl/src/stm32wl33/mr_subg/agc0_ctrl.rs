///Register `AGC0_CTRL` reader
pub type R = crate::R<AGC0_CTRLrs>;
///Register `AGC0_CTRL` writer
pub type W = crate::W<AGC0_CTRLrs>;
///Field `AGC_HOLD_TIME` reader - AGC hold time.
pub type AGC_HOLD_TIME_R = crate::FieldReader;
///Field `AGC_HOLD_TIME` writer - AGC hold time.
pub type AGC_HOLD_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `AGC_START_ONHOLD` reader - Start the AGC with a hold phase.
pub type AGC_START_ONHOLD_R = crate::BitReader;
///Field `AGC_START_ONHOLD` writer - Start the AGC with a hold phase.
pub type AGC_START_ONHOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AGC_EN` reader - Enable the AGC
pub type AGC_EN_R = crate::BitReader;
///Field `AGC_EN` writer - Enable the AGC
pub type AGC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:5 - AGC hold time.
    #[inline(always)]
    pub fn agc_hold_time(&self) -> AGC_HOLD_TIME_R {
        AGC_HOLD_TIME_R::new((self.bits & 0x3f) as u8)
    }
    ///Bit 6 - Start the AGC with a hold phase.
    #[inline(always)]
    pub fn agc_start_onhold(&self) -> AGC_START_ONHOLD_R {
        AGC_START_ONHOLD_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Enable the AGC
    #[inline(always)]
    pub fn agc_en(&self) -> AGC_EN_R {
        AGC_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AGC0_CTRL")
            .field("agc_hold_time", &self.agc_hold_time())
            .field("agc_start_onhold", &self.agc_start_onhold())
            .field("agc_en", &self.agc_en())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - AGC hold time.
    #[inline(always)]
    pub fn agc_hold_time(&mut self) -> AGC_HOLD_TIME_W<'_, AGC0_CTRLrs> {
        AGC_HOLD_TIME_W::new(self, 0)
    }
    ///Bit 6 - Start the AGC with a hold phase.
    #[inline(always)]
    pub fn agc_start_onhold(&mut self) -> AGC_START_ONHOLD_W<'_, AGC0_CTRLrs> {
        AGC_START_ONHOLD_W::new(self, 6)
    }
    ///Bit 7 - Enable the AGC
    #[inline(always)]
    pub fn agc_en(&mut self) -> AGC_EN_W<'_, AGC0_CTRLrs> {
        AGC_EN_W::new(self, 7)
    }
}
/**AGC0_CTRL register

You can [`read`](crate::Reg::read) this register and get [`agc0_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc0_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:AGC0_CTRL)*/
pub struct AGC0_CTRLrs;
impl crate::RegisterSpec for AGC0_CTRLrs {
    type Ux = u32;
}
///`read()` method returns [`agc0_ctrl::R`](R) reader structure
impl crate::Readable for AGC0_CTRLrs {}
///`write(|w| ..)` method takes [`agc0_ctrl::W`](W) writer structure
impl crate::Writable for AGC0_CTRLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AGC0_CTRL to value 0x99
impl crate::Resettable for AGC0_CTRLrs {
    const RESET_VALUE: u32 = 0x99;
}
