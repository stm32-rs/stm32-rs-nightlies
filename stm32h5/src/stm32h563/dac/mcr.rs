///Register `MCR` reader
pub type R = crate::R<MCRrs>;
///Register `MCR` writer
pub type W = crate::W<MCRrs>;
///Field `MODE1` reader - DAC channel1 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN1 = 0 and bit CEN1 = 0 in the DAC_CR register). If EN1 = 1 or CEN1 = 1 the write operation is ignored. They can be set and cleared by software to select the DAC channel1 mode: DAC channel1 in Normal mode DAC channel1 in sample & hold mode Note: This register can be modified only when EN1 = 0.
pub type MODE1_R = crate::FieldReader;
///Field `MODE1` writer - DAC channel1 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN1 = 0 and bit CEN1 = 0 in the DAC_CR register). If EN1 = 1 or CEN1 = 1 the write operation is ignored. They can be set and cleared by software to select the DAC channel1 mode: DAC channel1 in Normal mode DAC channel1 in sample & hold mode Note: This register can be modified only when EN1 = 0.
pub type MODE1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DMADOUBLE1` reader - DAC channel1 DMA double data mode This bit is set and cleared by software.
pub type DMADOUBLE1_R = crate::BitReader;
///Field `DMADOUBLE1` writer - DAC channel1 DMA double data mode This bit is set and cleared by software.
pub type DMADOUBLE1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SINFORMAT1` reader - Enable signed format for DAC channel1 This bit is set and cleared by software.
pub type SINFORMAT1_R = crate::BitReader;
///Field `SINFORMAT1` writer - Enable signed format for DAC channel1 This bit is set and cleared by software.
pub type SINFORMAT1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HFSEL0` reader - High frequency interface mode selection
pub type HFSEL0_R = crate::BitReader;
///Field `HFSEL0` writer - High frequency interface mode selection
pub type HFSEL0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HFSEL1` reader - High frequency interface mode selection
pub type HFSEL1_R = crate::BitReader;
///Field `HFSEL1` writer - High frequency interface mode selection
pub type HFSEL1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MODE2` reader - DAC channel2 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN2 = 0 and bit CEN2 = 0 in the DAC_CR register). If EN2 = 1 or CEN2 = 1 the write operation is ignored. They can be set and cleared by software to select the DAC channel2 mode: DAC channel2 in Normal mode DAC channel2 in Sample and hold mode Note: This register can be modified only when EN2 = 0. Refer to for the availability of DAC channel2.
pub type MODE2_R = crate::FieldReader;
///Field `MODE2` writer - DAC channel2 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN2 = 0 and bit CEN2 = 0 in the DAC_CR register). If EN2 = 1 or CEN2 = 1 the write operation is ignored. They can be set and cleared by software to select the DAC channel2 mode: DAC channel2 in Normal mode DAC channel2 in Sample and hold mode Note: This register can be modified only when EN2 = 0. Refer to for the availability of DAC channel2.
pub type MODE2_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DMADOUBLE2` reader - DAC channel2 DMA double data mode This bit is set and cleared by software. Note: This bit is available only on dual-channel DACs. Refer to implementation.
pub type DMADOUBLE2_R = crate::BitReader;
///Field `DMADOUBLE2` writer - DAC channel2 DMA double data mode This bit is set and cleared by software. Note: This bit is available only on dual-channel DACs. Refer to implementation.
pub type DMADOUBLE2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SINFORMAT2` reader - Enable signed format for DAC channel2 This bit is set and cleared by software. Note: This bit is available only on dual-channel DACs. Refer to implementation.
pub type SINFORMAT2_R = crate::BitReader;
///Field `SINFORMAT2` writer - Enable signed format for DAC channel2 This bit is set and cleared by software. Note: This bit is available only on dual-channel DACs. Refer to implementation.
pub type SINFORMAT2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:2 - DAC channel1 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN1 = 0 and bit CEN1 = 0 in the DAC_CR register). If EN1 = 1 or CEN1 = 1 the write operation is ignored. They can be set and cleared by software to select the DAC channel1 mode: DAC channel1 in Normal mode DAC channel1 in sample & hold mode Note: This register can be modified only when EN1 = 0.
    #[inline(always)]
    pub fn mode1(&self) -> MODE1_R {
        MODE1_R::new((self.bits & 7) as u8)
    }
    ///Bit 8 - DAC channel1 DMA double data mode This bit is set and cleared by software.
    #[inline(always)]
    pub fn dmadouble1(&self) -> DMADOUBLE1_R {
        DMADOUBLE1_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Enable signed format for DAC channel1 This bit is set and cleared by software.
    #[inline(always)]
    pub fn sinformat1(&self) -> SINFORMAT1_R {
        SINFORMAT1_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 14 - High frequency interface mode selection
    #[inline(always)]
    pub fn hfsel0(&self) -> HFSEL0_R {
        HFSEL0_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - High frequency interface mode selection
    #[inline(always)]
    pub fn hfsel1(&self) -> HFSEL1_R {
        HFSEL1_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:18 - DAC channel2 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN2 = 0 and bit CEN2 = 0 in the DAC_CR register). If EN2 = 1 or CEN2 = 1 the write operation is ignored. They can be set and cleared by software to select the DAC channel2 mode: DAC channel2 in Normal mode DAC channel2 in Sample and hold mode Note: This register can be modified only when EN2 = 0. Refer to for the availability of DAC channel2.
    #[inline(always)]
    pub fn mode2(&self) -> MODE2_R {
        MODE2_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bit 24 - DAC channel2 DMA double data mode This bit is set and cleared by software. Note: This bit is available only on dual-channel DACs. Refer to implementation.
    #[inline(always)]
    pub fn dmadouble2(&self) -> DMADOUBLE2_R {
        DMADOUBLE2_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Enable signed format for DAC channel2 This bit is set and cleared by software. Note: This bit is available only on dual-channel DACs. Refer to implementation.
    #[inline(always)]
    pub fn sinformat2(&self) -> SINFORMAT2_R {
        SINFORMAT2_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MCR")
            .field("mode1", &self.mode1())
            .field("dmadouble1", &self.dmadouble1())
            .field("sinformat1", &self.sinformat1())
            .field("hfsel0", &self.hfsel0())
            .field("hfsel1", &self.hfsel1())
            .field("mode2", &self.mode2())
            .field("dmadouble2", &self.dmadouble2())
            .field("sinformat2", &self.sinformat2())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - DAC channel1 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN1 = 0 and bit CEN1 = 0 in the DAC_CR register). If EN1 = 1 or CEN1 = 1 the write operation is ignored. They can be set and cleared by software to select the DAC channel1 mode: DAC channel1 in Normal mode DAC channel1 in sample & hold mode Note: This register can be modified only when EN1 = 0.
    #[inline(always)]
    pub fn mode1(&mut self) -> MODE1_W<MCRrs> {
        MODE1_W::new(self, 0)
    }
    ///Bit 8 - DAC channel1 DMA double data mode This bit is set and cleared by software.
    #[inline(always)]
    pub fn dmadouble1(&mut self) -> DMADOUBLE1_W<MCRrs> {
        DMADOUBLE1_W::new(self, 8)
    }
    ///Bit 9 - Enable signed format for DAC channel1 This bit is set and cleared by software.
    #[inline(always)]
    pub fn sinformat1(&mut self) -> SINFORMAT1_W<MCRrs> {
        SINFORMAT1_W::new(self, 9)
    }
    ///Bit 14 - High frequency interface mode selection
    #[inline(always)]
    pub fn hfsel0(&mut self) -> HFSEL0_W<MCRrs> {
        HFSEL0_W::new(self, 14)
    }
    ///Bit 15 - High frequency interface mode selection
    #[inline(always)]
    pub fn hfsel1(&mut self) -> HFSEL1_W<MCRrs> {
        HFSEL1_W::new(self, 15)
    }
    ///Bits 16:18 - DAC channel2 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN2 = 0 and bit CEN2 = 0 in the DAC_CR register). If EN2 = 1 or CEN2 = 1 the write operation is ignored. They can be set and cleared by software to select the DAC channel2 mode: DAC channel2 in Normal mode DAC channel2 in Sample and hold mode Note: This register can be modified only when EN2 = 0. Refer to for the availability of DAC channel2.
    #[inline(always)]
    pub fn mode2(&mut self) -> MODE2_W<MCRrs> {
        MODE2_W::new(self, 16)
    }
    ///Bit 24 - DAC channel2 DMA double data mode This bit is set and cleared by software. Note: This bit is available only on dual-channel DACs. Refer to implementation.
    #[inline(always)]
    pub fn dmadouble2(&mut self) -> DMADOUBLE2_W<MCRrs> {
        DMADOUBLE2_W::new(self, 24)
    }
    ///Bit 25 - Enable signed format for DAC channel2 This bit is set and cleared by software. Note: This bit is available only on dual-channel DACs. Refer to implementation.
    #[inline(always)]
    pub fn sinformat2(&mut self) -> SINFORMAT2_W<MCRrs> {
        SINFORMAT2_W::new(self, 25)
    }
}
/**DAC mode control register

You can [`read`](crate::Reg::read) this register and get [`mcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#DAC:MCR)*/
pub struct MCRrs;
impl crate::RegisterSpec for MCRrs {
    type Ux = u32;
}
///`read()` method returns [`mcr::R`](R) reader structure
impl crate::Readable for MCRrs {}
///`write(|w| ..)` method takes [`mcr::W`](W) writer structure
impl crate::Writable for MCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MCR to value 0
impl crate::Resettable for MCRrs {}
