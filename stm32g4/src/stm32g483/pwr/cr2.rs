///Register `CR2` reader
pub type R = crate::R<CR2rs>;
///Register `CR2` writer
pub type W = crate::W<CR2rs>;
///Field `PVDE` reader - Power voltage detector enable
pub type PVDE_R = crate::BitReader;
///Field `PVDE` writer - Power voltage detector enable
pub type PVDE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLS` reader - Power voltage detector level selection
pub type PLS_R = crate::FieldReader;
///Field `PLS` writer - Power voltage detector level selection
pub type PLS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `PVMEN1` reader - Peripheral voltage monitoring 1 enable: VDDA vs. COMP min voltage
pub type PVMEN1_R = crate::BitReader;
///Field `PVMEN1` writer - Peripheral voltage monitoring 1 enable: VDDA vs. COMP min voltage
pub type PVMEN1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PVMEN2` reader - Peripheral voltage monitoring 2 enable: VDDA vs. Fast DAC min voltage
pub type PVMEN2_R = crate::BitReader;
///Field `PVMEN2` writer - Peripheral voltage monitoring 2 enable: VDDA vs. Fast DAC min voltage
pub type PVMEN2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PVMEN3` reader - Peripheral voltage monitoring 3 enable: VDDA vs. ADC min voltage 1.62V
pub type PVMEN3_R = crate::BitReader;
///Field `PVMEN3` writer - Peripheral voltage monitoring 3 enable: VDDA vs. ADC min voltage 1.62V
pub type PVMEN3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PVMEN4` reader - Peripheral voltage monitoring 4 enable: VDDA vs. OPAMP/DAC min voltage
pub type PVMEN4_R = crate::BitReader;
///Field `PVMEN4` writer - Peripheral voltage monitoring 4 enable: VDDA vs. OPAMP/DAC min voltage
pub type PVMEN4_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Power voltage detector enable
    #[inline(always)]
    pub fn pvde(&self) -> PVDE_R {
        PVDE_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:3 - Power voltage detector level selection
    #[inline(always)]
    pub fn pls(&self) -> PLS_R {
        PLS_R::new(((self.bits >> 1) & 7) as u8)
    }
    ///Bit 4 - Peripheral voltage monitoring 1 enable: VDDA vs. COMP min voltage
    #[inline(always)]
    pub fn pvmen1(&self) -> PVMEN1_R {
        PVMEN1_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Peripheral voltage monitoring 2 enable: VDDA vs. Fast DAC min voltage
    #[inline(always)]
    pub fn pvmen2(&self) -> PVMEN2_R {
        PVMEN2_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Peripheral voltage monitoring 3 enable: VDDA vs. ADC min voltage 1.62V
    #[inline(always)]
    pub fn pvmen3(&self) -> PVMEN3_R {
        PVMEN3_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Peripheral voltage monitoring 4 enable: VDDA vs. OPAMP/DAC min voltage
    #[inline(always)]
    pub fn pvmen4(&self) -> PVMEN4_R {
        PVMEN4_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR2")
            .field("pvmen1", &self.pvmen1())
            .field("pls", &self.pls())
            .field("pvde", &self.pvde())
            .field("pvmen2", &self.pvmen2())
            .field("pvmen3", &self.pvmen3())
            .field("pvmen4", &self.pvmen4())
            .finish()
    }
}
impl W {
    ///Bit 0 - Power voltage detector enable
    #[inline(always)]
    pub fn pvde(&mut self) -> PVDE_W<CR2rs> {
        PVDE_W::new(self, 0)
    }
    ///Bits 1:3 - Power voltage detector level selection
    #[inline(always)]
    pub fn pls(&mut self) -> PLS_W<CR2rs> {
        PLS_W::new(self, 1)
    }
    ///Bit 4 - Peripheral voltage monitoring 1 enable: VDDA vs. COMP min voltage
    #[inline(always)]
    pub fn pvmen1(&mut self) -> PVMEN1_W<CR2rs> {
        PVMEN1_W::new(self, 4)
    }
    ///Bit 5 - Peripheral voltage monitoring 2 enable: VDDA vs. Fast DAC min voltage
    #[inline(always)]
    pub fn pvmen2(&mut self) -> PVMEN2_W<CR2rs> {
        PVMEN2_W::new(self, 5)
    }
    ///Bit 6 - Peripheral voltage monitoring 3 enable: VDDA vs. ADC min voltage 1.62V
    #[inline(always)]
    pub fn pvmen3(&mut self) -> PVMEN3_W<CR2rs> {
        PVMEN3_W::new(self, 6)
    }
    ///Bit 7 - Peripheral voltage monitoring 4 enable: VDDA vs. OPAMP/DAC min voltage
    #[inline(always)]
    pub fn pvmen4(&mut self) -> PVMEN4_W<CR2rs> {
        PVMEN4_W::new(self, 7)
    }
}
/**Power control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G483xx.html#PWR:CR2)*/
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
///`read()` method returns [`cr2::R`](R) reader structure
impl crate::Readable for CR2rs {}
///`write(|w| ..)` method takes [`cr2::W`](W) writer structure
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CR2 to value 0
impl crate::Resettable for CR2rs {
    const RESET_VALUE: u32 = 0;
}
