///Register `CR2` reader
pub type R = crate::R<CR2rs>;
///Register `CR2` writer
pub type W = crate::W<CR2rs>;
///Field `TAMP1NOER` reader - Tamper 1 no erase
pub type TAMP1NOER_R = crate::BitReader;
///Field `TAMP1NOER` writer - Tamper 1 no erase
pub type TAMP1NOER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP2NOER` reader - Tamper 2 no erase
pub type TAMP2NOER_R = crate::BitReader;
///Field `TAMP2NOER` writer - Tamper 2 no erase
pub type TAMP2NOER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP1MSK` reader - Tamper 1 mask The tamper 1 interrupt must not be enabled when TAMP1MSK is set.
pub type TAMP1MSK_R = crate::BitReader;
///Field `TAMP1MSK` writer - Tamper 1 mask The tamper 1 interrupt must not be enabled when TAMP1MSK is set.
pub type TAMP1MSK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP2MSK` reader - Tamper 2 mask The tamper 2 interrupt must not be enabled when TAMP2MSK is set.
pub type TAMP2MSK_R = crate::BitReader;
///Field `TAMP2MSK` writer - Tamper 2 mask The tamper 2 interrupt must not be enabled when TAMP2MSK is set.
pub type TAMP2MSK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP1TRG` reader - Active level for tamper 1 input (active mode disabled) If TAMPFLT = 00 Tamper 1 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 1 input falling edge and low level triggers a tamper detection event.
pub type TAMP1TRG_R = crate::BitReader;
///Field `TAMP1TRG` writer - Active level for tamper 1 input (active mode disabled) If TAMPFLT = 00 Tamper 1 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 1 input falling edge and low level triggers a tamper detection event.
pub type TAMP1TRG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP2TRG` reader - Active level for tamper 2 input (active mode disabled) If TAMPFLT = 00 Tamper 2 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 2 input falling edge and low level triggers a tamper detection event.
pub type TAMP2TRG_R = crate::BitReader;
///Field `TAMP2TRG` writer - Active level for tamper 2 input (active mode disabled) If TAMPFLT = 00 Tamper 2 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 2 input falling edge and low level triggers a tamper detection event.
pub type TAMP2TRG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Tamper 1 no erase
    #[inline(always)]
    pub fn tamp1noer(&self) -> TAMP1NOER_R {
        TAMP1NOER_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Tamper 2 no erase
    #[inline(always)]
    pub fn tamp2noer(&self) -> TAMP2NOER_R {
        TAMP2NOER_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 16 - Tamper 1 mask The tamper 1 interrupt must not be enabled when TAMP1MSK is set.
    #[inline(always)]
    pub fn tamp1msk(&self) -> TAMP1MSK_R {
        TAMP1MSK_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Tamper 2 mask The tamper 2 interrupt must not be enabled when TAMP2MSK is set.
    #[inline(always)]
    pub fn tamp2msk(&self) -> TAMP2MSK_R {
        TAMP2MSK_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 24 - Active level for tamper 1 input (active mode disabled) If TAMPFLT = 00 Tamper 1 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 1 input falling edge and low level triggers a tamper detection event.
    #[inline(always)]
    pub fn tamp1trg(&self) -> TAMP1TRG_R {
        TAMP1TRG_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Active level for tamper 2 input (active mode disabled) If TAMPFLT = 00 Tamper 2 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 2 input falling edge and low level triggers a tamper detection event.
    #[inline(always)]
    pub fn tamp2trg(&self) -> TAMP2TRG_R {
        TAMP2TRG_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR2")
            .field("tamp1noer", &self.tamp1noer())
            .field("tamp2noer", &self.tamp2noer())
            .field("tamp1msk", &self.tamp1msk())
            .field("tamp2msk", &self.tamp2msk())
            .field("tamp1trg", &self.tamp1trg())
            .field("tamp2trg", &self.tamp2trg())
            .finish()
    }
}
impl W {
    ///Bit 0 - Tamper 1 no erase
    #[inline(always)]
    pub fn tamp1noer(&mut self) -> TAMP1NOER_W<'_, CR2rs> {
        TAMP1NOER_W::new(self, 0)
    }
    ///Bit 1 - Tamper 2 no erase
    #[inline(always)]
    pub fn tamp2noer(&mut self) -> TAMP2NOER_W<'_, CR2rs> {
        TAMP2NOER_W::new(self, 1)
    }
    ///Bit 16 - Tamper 1 mask The tamper 1 interrupt must not be enabled when TAMP1MSK is set.
    #[inline(always)]
    pub fn tamp1msk(&mut self) -> TAMP1MSK_W<'_, CR2rs> {
        TAMP1MSK_W::new(self, 16)
    }
    ///Bit 17 - Tamper 2 mask The tamper 2 interrupt must not be enabled when TAMP2MSK is set.
    #[inline(always)]
    pub fn tamp2msk(&mut self) -> TAMP2MSK_W<'_, CR2rs> {
        TAMP2MSK_W::new(self, 17)
    }
    ///Bit 24 - Active level for tamper 1 input (active mode disabled) If TAMPFLT = 00 Tamper 1 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 1 input falling edge and low level triggers a tamper detection event.
    #[inline(always)]
    pub fn tamp1trg(&mut self) -> TAMP1TRG_W<'_, CR2rs> {
        TAMP1TRG_W::new(self, 24)
    }
    ///Bit 25 - Active level for tamper 2 input (active mode disabled) If TAMPFLT = 00 Tamper 2 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 2 input falling edge and low level triggers a tamper detection event.
    #[inline(always)]
    pub fn tamp2trg(&mut self) -> TAMP2TRG_W<'_, CR2rs> {
        TAMP2TRG_W::new(self, 25)
    }
}
/**TAMP control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0B1.html#TAMP:CR2)*/
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
///`read()` method returns [`cr2::R`](R) reader structure
impl crate::Readable for CR2rs {}
///`write(|w| ..)` method takes [`cr2::W`](W) writer structure
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR2 to value 0
impl crate::Resettable for CR2rs {}
