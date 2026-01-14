///Register `CFGR1` reader
pub type R = crate::R<CFGR1rs>;
///Register `CFGR1` writer
pub type W = crate::W<CFGR1rs>;
///Field `FWDIS` reader - Firewall disable
pub type FWDIS_R = crate::BitReader;
///Field `FWDIS` writer - Firewall disable
pub type FWDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BOOSTEN` reader - I/O analog switch voltage booster enable
pub type BOOSTEN_R = crate::BitReader;
///Field `BOOSTEN` writer - I/O analog switch voltage booster enable
pub type BOOSTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C_PB6_FMP` reader - Fast-mode Plus (Fm+) driving capability activation on PB6
pub type I2C_PB6_FMP_R = crate::BitReader;
///Field `I2C_PB6_FMP` writer - Fast-mode Plus (Fm+) driving capability activation on PB6
pub type I2C_PB6_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C_PB7_FMP` reader - Fast-mode Plus (Fm+) driving capability activation on PB7
pub type I2C_PB7_FMP_R = crate::BitReader;
///Field `I2C_PB7_FMP` writer - Fast-mode Plus (Fm+) driving capability activation on PB7
pub type I2C_PB7_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C_PB8_FMP` reader - Fast-mode Plus (Fm+) driving capability activation on PB8
pub type I2C_PB8_FMP_R = crate::BitReader;
///Field `I2C_PB8_FMP` writer - Fast-mode Plus (Fm+) driving capability activation on PB8
pub type I2C_PB8_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C_PB9_FMP` reader - Fast-mode Plus (Fm+) driving capability activation on PB9
pub type I2C_PB9_FMP_R = crate::BitReader;
///Field `I2C_PB9_FMP` writer - Fast-mode Plus (Fm+) driving capability activation on PB9
pub type I2C_PB9_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C1_FMP` reader - I2C1 Fast-mode Plus driving capability activation
pub type I2C1_FMP_R = crate::BitReader;
///Field `I2C1_FMP` writer - I2C1 Fast-mode Plus driving capability activation
pub type I2C1_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C2_FMP` reader - I2C2 Fast-mode Plus driving capability activation
pub type I2C2_FMP_R = crate::BitReader;
///Field `I2C2_FMP` writer - I2C2 Fast-mode Plus driving capability activation
pub type I2C2_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C3_FMP` reader - I2C3 Fast-mode Plus driving capability activation
pub type I2C3_FMP_R = crate::BitReader;
///Field `I2C3_FMP` writer - I2C3 Fast-mode Plus driving capability activation
pub type I2C3_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPU_IE` reader - Floating Point Unit interrupts enable bits
pub type FPU_IE_R = crate::FieldReader;
///Field `FPU_IE` writer - Floating Point Unit interrupts enable bits
pub type FPU_IE_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bit 0 - Firewall disable
    #[inline(always)]
    pub fn fwdis(&self) -> FWDIS_R {
        FWDIS_R::new((self.bits & 1) != 0)
    }
    ///Bit 8 - I/O analog switch voltage booster enable
    #[inline(always)]
    pub fn boosten(&self) -> BOOSTEN_R {
        BOOSTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 16 - Fast-mode Plus (Fm+) driving capability activation on PB6
    #[inline(always)]
    pub fn i2c_pb6_fmp(&self) -> I2C_PB6_FMP_R {
        I2C_PB6_FMP_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Fast-mode Plus (Fm+) driving capability activation on PB7
    #[inline(always)]
    pub fn i2c_pb7_fmp(&self) -> I2C_PB7_FMP_R {
        I2C_PB7_FMP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Fast-mode Plus (Fm+) driving capability activation on PB8
    #[inline(always)]
    pub fn i2c_pb8_fmp(&self) -> I2C_PB8_FMP_R {
        I2C_PB8_FMP_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Fast-mode Plus (Fm+) driving capability activation on PB9
    #[inline(always)]
    pub fn i2c_pb9_fmp(&self) -> I2C_PB9_FMP_R {
        I2C_PB9_FMP_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - I2C1 Fast-mode Plus driving capability activation
    #[inline(always)]
    pub fn i2c1_fmp(&self) -> I2C1_FMP_R {
        I2C1_FMP_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - I2C2 Fast-mode Plus driving capability activation
    #[inline(always)]
    pub fn i2c2_fmp(&self) -> I2C2_FMP_R {
        I2C2_FMP_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - I2C3 Fast-mode Plus driving capability activation
    #[inline(always)]
    pub fn i2c3_fmp(&self) -> I2C3_FMP_R {
        I2C3_FMP_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bits 26:31 - Floating Point Unit interrupts enable bits
    #[inline(always)]
    pub fn fpu_ie(&self) -> FPU_IE_R {
        FPU_IE_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR1")
            .field("fpu_ie", &self.fpu_ie())
            .field("i2c3_fmp", &self.i2c3_fmp())
            .field("i2c2_fmp", &self.i2c2_fmp())
            .field("i2c1_fmp", &self.i2c1_fmp())
            .field("i2c_pb9_fmp", &self.i2c_pb9_fmp())
            .field("i2c_pb8_fmp", &self.i2c_pb8_fmp())
            .field("i2c_pb7_fmp", &self.i2c_pb7_fmp())
            .field("i2c_pb6_fmp", &self.i2c_pb6_fmp())
            .field("boosten", &self.boosten())
            .field("fwdis", &self.fwdis())
            .finish()
    }
}
impl W {
    ///Bit 0 - Firewall disable
    #[inline(always)]
    pub fn fwdis(&mut self) -> FWDIS_W<'_, CFGR1rs> {
        FWDIS_W::new(self, 0)
    }
    ///Bit 8 - I/O analog switch voltage booster enable
    #[inline(always)]
    pub fn boosten(&mut self) -> BOOSTEN_W<'_, CFGR1rs> {
        BOOSTEN_W::new(self, 8)
    }
    ///Bit 16 - Fast-mode Plus (Fm+) driving capability activation on PB6
    #[inline(always)]
    pub fn i2c_pb6_fmp(&mut self) -> I2C_PB6_FMP_W<'_, CFGR1rs> {
        I2C_PB6_FMP_W::new(self, 16)
    }
    ///Bit 17 - Fast-mode Plus (Fm+) driving capability activation on PB7
    #[inline(always)]
    pub fn i2c_pb7_fmp(&mut self) -> I2C_PB7_FMP_W<'_, CFGR1rs> {
        I2C_PB7_FMP_W::new(self, 17)
    }
    ///Bit 18 - Fast-mode Plus (Fm+) driving capability activation on PB8
    #[inline(always)]
    pub fn i2c_pb8_fmp(&mut self) -> I2C_PB8_FMP_W<'_, CFGR1rs> {
        I2C_PB8_FMP_W::new(self, 18)
    }
    ///Bit 19 - Fast-mode Plus (Fm+) driving capability activation on PB9
    #[inline(always)]
    pub fn i2c_pb9_fmp(&mut self) -> I2C_PB9_FMP_W<'_, CFGR1rs> {
        I2C_PB9_FMP_W::new(self, 19)
    }
    ///Bit 20 - I2C1 Fast-mode Plus driving capability activation
    #[inline(always)]
    pub fn i2c1_fmp(&mut self) -> I2C1_FMP_W<'_, CFGR1rs> {
        I2C1_FMP_W::new(self, 20)
    }
    ///Bit 21 - I2C2 Fast-mode Plus driving capability activation
    #[inline(always)]
    pub fn i2c2_fmp(&mut self) -> I2C2_FMP_W<'_, CFGR1rs> {
        I2C2_FMP_W::new(self, 21)
    }
    ///Bit 22 - I2C3 Fast-mode Plus driving capability activation
    #[inline(always)]
    pub fn i2c3_fmp(&mut self) -> I2C3_FMP_W<'_, CFGR1rs> {
        I2C3_FMP_W::new(self, 22)
    }
    ///Bits 26:31 - Floating Point Unit interrupts enable bits
    #[inline(always)]
    pub fn fpu_ie(&mut self) -> FPU_IE_W<'_, CFGR1rs> {
        FPU_IE_W::new(self, 26)
    }
}
/**configuration register 1

You can [`read`](crate::Reg::read) this register and get [`cfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x5.html#SYSCFG:CFGR1)*/
pub struct CFGR1rs;
impl crate::RegisterSpec for CFGR1rs {
    type Ux = u32;
}
///`read()` method returns [`cfgr1::R`](R) reader structure
impl crate::Readable for CFGR1rs {}
///`write(|w| ..)` method takes [`cfgr1::W`](W) writer structure
impl crate::Writable for CFGR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR1 to value 0x7c00_0001
impl crate::Resettable for CFGR1rs {
    const RESET_VALUE: u32 = 0x7c00_0001;
}
