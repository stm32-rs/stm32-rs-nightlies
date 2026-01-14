///Register `CR2` reader
pub type R = crate::R<CR2rs>;
///Register `CR2` writer
pub type W = crate::W<CR2rs>;
///Field `PVDE` reader - Programmable voltage detector enable
pub type PVDE_R = crate::BitReader;
///Field `PVDE` writer - Programmable voltage detector enable
pub type PVDE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PVDLS` reader - Programmable voltage detector level selection.
pub type PVDLS_R = crate::FieldReader;
///Field `PVDLS` writer - Programmable voltage detector level selection.
pub type PVDLS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `PVMEN1` reader - Peripheral voltage monitoring 3 enable: V<sub>DDA</sub> vs. ADC/COMP min voltage 1.62V
pub type PVMEN1_R = crate::BitReader;
///Field `PVMEN1` writer - Peripheral voltage monitoring 3 enable: V<sub>DDA</sub> vs. ADC/COMP min voltage 1.62V
pub type PVMEN1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PVMEN2` reader - Peripheral voltage monitoring 4 enable: V<sub>DDA</sub> vs. DAC 1MSPS /DAC 15MSPS min voltage.
pub type PVMEN2_R = crate::BitReader;
///Field `PVMEN2` writer - Peripheral voltage monitoring 4 enable: V<sub>DDA</sub> vs. DAC 1MSPS /DAC 15MSPS min voltage.
pub type PVMEN2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Programmable voltage detector enable
    #[inline(always)]
    pub fn pvde(&self) -> PVDE_R {
        PVDE_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:3 - Programmable voltage detector level selection.
    #[inline(always)]
    pub fn pvdls(&self) -> PVDLS_R {
        PVDLS_R::new(((self.bits >> 1) & 7) as u8)
    }
    ///Bit 6 - Peripheral voltage monitoring 3 enable: V<sub>DDA</sub> vs. ADC/COMP min voltage 1.62V
    #[inline(always)]
    pub fn pvmen1(&self) -> PVMEN1_R {
        PVMEN1_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Peripheral voltage monitoring 4 enable: V<sub>DDA</sub> vs. DAC 1MSPS /DAC 15MSPS min voltage.
    #[inline(always)]
    pub fn pvmen2(&self) -> PVMEN2_R {
        PVMEN2_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR2")
            .field("pvde", &self.pvde())
            .field("pvdls", &self.pvdls())
            .field("pvmen1", &self.pvmen1())
            .field("pvmen2", &self.pvmen2())
            .finish()
    }
}
impl W {
    ///Bit 0 - Programmable voltage detector enable
    #[inline(always)]
    pub fn pvde(&mut self) -> PVDE_W<'_, CR2rs> {
        PVDE_W::new(self, 0)
    }
    ///Bits 1:3 - Programmable voltage detector level selection.
    #[inline(always)]
    pub fn pvdls(&mut self) -> PVDLS_W<'_, CR2rs> {
        PVDLS_W::new(self, 1)
    }
    ///Bit 6 - Peripheral voltage monitoring 3 enable: V<sub>DDA</sub> vs. ADC/COMP min voltage 1.62V
    #[inline(always)]
    pub fn pvmen1(&mut self) -> PVMEN1_W<'_, CR2rs> {
        PVMEN1_W::new(self, 6)
    }
    ///Bit 7 - Peripheral voltage monitoring 4 enable: V<sub>DDA</sub> vs. DAC 1MSPS /DAC 15MSPS min voltage.
    #[inline(always)]
    pub fn pvmen2(&mut self) -> PVMEN2_W<'_, CR2rs> {
        PVMEN2_W::new(self, 7)
    }
}
/**Power control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G484.html#PWR:CR2)*/
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
