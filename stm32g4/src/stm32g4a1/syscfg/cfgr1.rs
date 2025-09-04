///Register `CFGR1` reader
pub type R = crate::R<CFGR1rs>;
///Register `CFGR1` writer
pub type W = crate::W<CFGR1rs>;
///Field `BOOSTEN` reader - BOOSTEN
pub type BOOSTEN_R = crate::BitReader;
///Field `BOOSTEN` writer - BOOSTEN
pub type BOOSTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ANASWVDD` reader - GPIO analog switch control voltage selection
pub type ANASWVDD_R = crate::BitReader;
///Field `ANASWVDD` writer - GPIO analog switch control voltage selection
pub type ANASWVDD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C_PB6_FMP` reader - FM+ drive capability on PB6
pub type I2C_PB6_FMP_R = crate::BitReader;
///Field `I2C_PB6_FMP` writer - FM+ drive capability on PB6
pub type I2C_PB6_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C_PB7_FMP` reader - FM+ drive capability on PB6
pub type I2C_PB7_FMP_R = crate::BitReader;
///Field `I2C_PB7_FMP` writer - FM+ drive capability on PB6
pub type I2C_PB7_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C_PB8_FMP` reader - FM+ drive capability on PB6
pub type I2C_PB8_FMP_R = crate::BitReader;
///Field `I2C_PB8_FMP` writer - FM+ drive capability on PB6
pub type I2C_PB8_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C_PB9_FMP` reader - FM+ drive capability on PB6
pub type I2C_PB9_FMP_R = crate::BitReader;
///Field `I2C_PB9_FMP` writer - FM+ drive capability on PB6
pub type I2C_PB9_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C1_FMP` reader - I2C1 FM+ drive capability enable
pub type I2C1_FMP_R = crate::BitReader;
///Field `I2C1_FMP` writer - I2C1 FM+ drive capability enable
pub type I2C1_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C2_FMP` reader - I2C1 FM+ drive capability enable
pub type I2C2_FMP_R = crate::BitReader;
///Field `I2C2_FMP` writer - I2C1 FM+ drive capability enable
pub type I2C2_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C3_FMP` reader - I2C1 FM+ drive capability enable
pub type I2C3_FMP_R = crate::BitReader;
///Field `I2C3_FMP` writer - I2C1 FM+ drive capability enable
pub type I2C3_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C4_FMP` reader - I2C1 FM+ drive capability enable
pub type I2C4_FMP_R = crate::BitReader;
///Field `I2C4_FMP` writer - I2C1 FM+ drive capability enable
pub type I2C4_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPU_IE` reader - FPU Interrupts Enable
pub type FPU_IE_R = crate::FieldReader;
///Field `FPU_IE` writer - FPU Interrupts Enable
pub type FPU_IE_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bit 8 - BOOSTEN
    #[inline(always)]
    pub fn boosten(&self) -> BOOSTEN_R {
        BOOSTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - GPIO analog switch control voltage selection
    #[inline(always)]
    pub fn anaswvdd(&self) -> ANASWVDD_R {
        ANASWVDD_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 16 - FM+ drive capability on PB6
    #[inline(always)]
    pub fn i2c_pb6_fmp(&self) -> I2C_PB6_FMP_R {
        I2C_PB6_FMP_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - FM+ drive capability on PB6
    #[inline(always)]
    pub fn i2c_pb7_fmp(&self) -> I2C_PB7_FMP_R {
        I2C_PB7_FMP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - FM+ drive capability on PB6
    #[inline(always)]
    pub fn i2c_pb8_fmp(&self) -> I2C_PB8_FMP_R {
        I2C_PB8_FMP_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - FM+ drive capability on PB6
    #[inline(always)]
    pub fn i2c_pb9_fmp(&self) -> I2C_PB9_FMP_R {
        I2C_PB9_FMP_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - I2C1 FM+ drive capability enable
    #[inline(always)]
    pub fn i2c1_fmp(&self) -> I2C1_FMP_R {
        I2C1_FMP_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - I2C1 FM+ drive capability enable
    #[inline(always)]
    pub fn i2c2_fmp(&self) -> I2C2_FMP_R {
        I2C2_FMP_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - I2C1 FM+ drive capability enable
    #[inline(always)]
    pub fn i2c3_fmp(&self) -> I2C3_FMP_R {
        I2C3_FMP_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - I2C1 FM+ drive capability enable
    #[inline(always)]
    pub fn i2c4_fmp(&self) -> I2C4_FMP_R {
        I2C4_FMP_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 26:31 - FPU Interrupts Enable
    #[inline(always)]
    pub fn fpu_ie(&self) -> FPU_IE_R {
        FPU_IE_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR1")
            .field("boosten", &self.boosten())
            .field("anaswvdd", &self.anaswvdd())
            .field("i2c_pb6_fmp", &self.i2c_pb6_fmp())
            .field("i2c_pb7_fmp", &self.i2c_pb7_fmp())
            .field("i2c_pb8_fmp", &self.i2c_pb8_fmp())
            .field("i2c_pb9_fmp", &self.i2c_pb9_fmp())
            .field("i2c1_fmp", &self.i2c1_fmp())
            .field("i2c2_fmp", &self.i2c2_fmp())
            .field("i2c3_fmp", &self.i2c3_fmp())
            .field("i2c4_fmp", &self.i2c4_fmp())
            .field("fpu_ie", &self.fpu_ie())
            .finish()
    }
}
impl W {
    ///Bit 8 - BOOSTEN
    #[inline(always)]
    pub fn boosten(&mut self) -> BOOSTEN_W<CFGR1rs> {
        BOOSTEN_W::new(self, 8)
    }
    ///Bit 9 - GPIO analog switch control voltage selection
    #[inline(always)]
    pub fn anaswvdd(&mut self) -> ANASWVDD_W<CFGR1rs> {
        ANASWVDD_W::new(self, 9)
    }
    ///Bit 16 - FM+ drive capability on PB6
    #[inline(always)]
    pub fn i2c_pb6_fmp(&mut self) -> I2C_PB6_FMP_W<CFGR1rs> {
        I2C_PB6_FMP_W::new(self, 16)
    }
    ///Bit 17 - FM+ drive capability on PB6
    #[inline(always)]
    pub fn i2c_pb7_fmp(&mut self) -> I2C_PB7_FMP_W<CFGR1rs> {
        I2C_PB7_FMP_W::new(self, 17)
    }
    ///Bit 18 - FM+ drive capability on PB6
    #[inline(always)]
    pub fn i2c_pb8_fmp(&mut self) -> I2C_PB8_FMP_W<CFGR1rs> {
        I2C_PB8_FMP_W::new(self, 18)
    }
    ///Bit 19 - FM+ drive capability on PB6
    #[inline(always)]
    pub fn i2c_pb9_fmp(&mut self) -> I2C_PB9_FMP_W<CFGR1rs> {
        I2C_PB9_FMP_W::new(self, 19)
    }
    ///Bit 20 - I2C1 FM+ drive capability enable
    #[inline(always)]
    pub fn i2c1_fmp(&mut self) -> I2C1_FMP_W<CFGR1rs> {
        I2C1_FMP_W::new(self, 20)
    }
    ///Bit 21 - I2C1 FM+ drive capability enable
    #[inline(always)]
    pub fn i2c2_fmp(&mut self) -> I2C2_FMP_W<CFGR1rs> {
        I2C2_FMP_W::new(self, 21)
    }
    ///Bit 22 - I2C1 FM+ drive capability enable
    #[inline(always)]
    pub fn i2c3_fmp(&mut self) -> I2C3_FMP_W<CFGR1rs> {
        I2C3_FMP_W::new(self, 22)
    }
    ///Bit 23 - I2C1 FM+ drive capability enable
    #[inline(always)]
    pub fn i2c4_fmp(&mut self) -> I2C4_FMP_W<CFGR1rs> {
        I2C4_FMP_W::new(self, 23)
    }
    ///Bits 26:31 - FPU Interrupts Enable
    #[inline(always)]
    pub fn fpu_ie(&mut self) -> FPU_IE_W<CFGR1rs> {
        FPU_IE_W::new(self, 26)
    }
}
/**peripheral mode configuration register

You can [`read`](crate::Reg::read) this register and get [`cfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G4A1.html#SYSCFG:CFGR1)*/
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
