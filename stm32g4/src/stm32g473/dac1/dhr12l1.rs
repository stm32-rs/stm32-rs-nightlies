///Register `DHR12L1` reader
pub type R = crate::R<DHR12L1rs>;
///Register `DHR12L1` writer
pub type W = crate::W<DHR12L1rs>;
///Field `DACC1DHR` reader - DAC channel1 12-bit left-aligned data These bits are written by software which specifies 12-bit data for DAC channel1.
pub type DACC1DHR_R = crate::FieldReader<u16>;
///Field `DACC1DHR` writer - DAC channel1 12-bit left-aligned data These bits are written by software which specifies 12-bit data for DAC channel1.
pub type DACC1DHR_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16, crate::Safe>;
///Field `DACC1DHRB` reader - DAC channel1 12-bit left-aligned data B
pub type DACC1DHRB_R = crate::FieldReader<u16>;
///Field `DACC1DHRB` writer - DAC channel1 12-bit left-aligned data B
pub type DACC1DHRB_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 4:15 - DAC channel1 12-bit left-aligned data These bits are written by software which specifies 12-bit data for DAC channel1.
    #[inline(always)]
    pub fn dacc1dhr(&self) -> DACC1DHR_R {
        DACC1DHR_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    ///Bits 20:31 - DAC channel1 12-bit left-aligned data B
    #[inline(always)]
    pub fn dacc1dhrb(&self) -> DACC1DHRB_R {
        DACC1DHRB_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHR12L1")
            .field("dacc1dhr", &self.dacc1dhr())
            .field("dacc1dhrb", &self.dacc1dhrb())
            .finish()
    }
}
impl W {
    ///Bits 4:15 - DAC channel1 12-bit left-aligned data These bits are written by software which specifies 12-bit data for DAC channel1.
    #[inline(always)]
    pub fn dacc1dhr(&mut self) -> DACC1DHR_W<DHR12L1rs> {
        DACC1DHR_W::new(self, 4)
    }
    ///Bits 20:31 - DAC channel1 12-bit left-aligned data B
    #[inline(always)]
    pub fn dacc1dhrb(&mut self) -> DACC1DHRB_W<DHR12L1rs> {
        DACC1DHRB_W::new(self, 20)
    }
}
/**DAC channel1 12-bit left aligned data holding register

You can [`read`](crate::Reg::read) this register and get [`dhr12l1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12l1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G473.html#DAC1:DHR12L1)*/
pub struct DHR12L1rs;
impl crate::RegisterSpec for DHR12L1rs {
    type Ux = u32;
}
///`read()` method returns [`dhr12l1::R`](R) reader structure
impl crate::Readable for DHR12L1rs {}
///`write(|w| ..)` method takes [`dhr12l1::W`](W) writer structure
impl crate::Writable for DHR12L1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHR12L1 to value 0
impl crate::Resettable for DHR12L1rs {}
