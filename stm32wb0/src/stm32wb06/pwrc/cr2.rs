///Register `CR2` reader
pub type R = crate::R<CR2rs>;
///Register `CR2` writer
pub type W = crate::W<CR2rs>;
///Field `PVDE` reader - PVDE Programmable Voltage Detector Enable When this bit is set the Power Voltage Detector is enabled
pub type PVDE_R = crate::BitReader;
///Field `PVDE` writer - PVDE Programmable Voltage Detector Enable When this bit is set the Power Voltage Detector is enabled
pub type PVDE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PVDLS` reader - PVDLS\[2:0\] Programmable Voltage Detector Level selection then PVDO=1)
pub type PVDLS_R = crate::FieldReader;
///Field `PVDLS` writer - PVDLS\[2:0\] Programmable Voltage Detector Level selection then PVDO=1)
pub type PVDLS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `RAMRET1` reader - RAMRET1: RAM1 retention during low power mode
pub type RAMRET1_R = crate::BitReader;
///Field `RAMRET1` writer - RAMRET1: RAM1 retention during low power mode
pub type RAMRET1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RAMRET2` reader - Enables the RAM2 bank retention in DEEPSTOP mode.
pub type RAMRET2_R = crate::BitReader;
///Field `RAMRET2` writer - Enables the RAM2 bank retention in DEEPSTOP mode.
pub type RAMRET2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RAMRET3` reader - Enables the RAM3 bank retention in DEEPSTOP mode.
pub type RAMRET3_R = crate::BitReader;
///Field `RAMRET3` writer - Enables the RAM3 bank retention in DEEPSTOP mode.
pub type RAMRET3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ENTS` reader - Enable the temperature sensor.
pub type ENTS_R = crate::BitReader;
///Field `ENTS` writer - Enable the temperature sensor.
pub type ENTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSILPMUFEN` reader - LSI LPMU force enable.
pub type LSILPMUFEN_R = crate::BitReader;
///Field `LSILPMUFEN` writer - LSI LPMU force enable.
pub type LSILPMUFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - PVDE Programmable Voltage Detector Enable When this bit is set the Power Voltage Detector is enabled
    #[inline(always)]
    pub fn pvde(&self) -> PVDE_R {
        PVDE_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:3 - PVDLS\[2:0\] Programmable Voltage Detector Level selection then PVDO=1)
    #[inline(always)]
    pub fn pvdls(&self) -> PVDLS_R {
        PVDLS_R::new(((self.bits >> 1) & 7) as u8)
    }
    ///Bit 5 - RAMRET1: RAM1 retention during low power mode
    #[inline(always)]
    pub fn ramret1(&self) -> RAMRET1_R {
        RAMRET1_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Enables the RAM2 bank retention in DEEPSTOP mode.
    #[inline(always)]
    pub fn ramret2(&self) -> RAMRET2_R {
        RAMRET2_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Enables the RAM3 bank retention in DEEPSTOP mode.
    #[inline(always)]
    pub fn ramret3(&self) -> RAMRET3_R {
        RAMRET3_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 9 - Enable the temperature sensor.
    #[inline(always)]
    pub fn ents(&self) -> ENTS_R {
        ENTS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - LSI LPMU force enable.
    #[inline(always)]
    pub fn lsilpmufen(&self) -> LSILPMUFEN_R {
        LSILPMUFEN_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR2")
            .field("pvde", &self.pvde())
            .field("pvdls", &self.pvdls())
            .field("ramret1", &self.ramret1())
            .field("ramret2", &self.ramret2())
            .field("ramret3", &self.ramret3())
            .field("ents", &self.ents())
            .field("lsilpmufen", &self.lsilpmufen())
            .finish()
    }
}
impl W {
    ///Bit 0 - PVDE Programmable Voltage Detector Enable When this bit is set the Power Voltage Detector is enabled
    #[inline(always)]
    pub fn pvde(&mut self) -> PVDE_W<'_, CR2rs> {
        PVDE_W::new(self, 0)
    }
    ///Bits 1:3 - PVDLS\[2:0\] Programmable Voltage Detector Level selection then PVDO=1)
    #[inline(always)]
    pub fn pvdls(&mut self) -> PVDLS_W<'_, CR2rs> {
        PVDLS_W::new(self, 1)
    }
    ///Bit 5 - RAMRET1: RAM1 retention during low power mode
    #[inline(always)]
    pub fn ramret1(&mut self) -> RAMRET1_W<'_, CR2rs> {
        RAMRET1_W::new(self, 5)
    }
    ///Bit 6 - Enables the RAM2 bank retention in DEEPSTOP mode.
    #[inline(always)]
    pub fn ramret2(&mut self) -> RAMRET2_W<'_, CR2rs> {
        RAMRET2_W::new(self, 6)
    }
    ///Bit 7 - Enables the RAM3 bank retention in DEEPSTOP mode.
    #[inline(always)]
    pub fn ramret3(&mut self) -> RAMRET3_W<'_, CR2rs> {
        RAMRET3_W::new(self, 7)
    }
    ///Bit 9 - Enable the temperature sensor.
    #[inline(always)]
    pub fn ents(&mut self) -> ENTS_W<'_, CR2rs> {
        ENTS_W::new(self, 9)
    }
    ///Bit 10 - LSI LPMU force enable.
    #[inline(always)]
    pub fn lsilpmufen(&mut self) -> LSILPMUFEN_W<'_, CR2rs> {
        LSILPMUFEN_W::new(self, 10)
    }
}
/**CR2 register

You can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#PWRC:CR2)*/
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
