///Register `L2CYR0R` reader
pub type R = crate::R<L2CYR0Rrs>;
///Register `L2CYR0R` writer
pub type W = crate::W<L2CYR0Rrs>;
///Field `CR2R` reader - Cr-to-Red coefficient, with bits 9:8 as positive integer and 7:0 as decimals.
pub type CR2R_R = crate::FieldReader<u16>;
///Field `CR2R` writer - Cr-to-Red coefficient, with bits 9:8 as positive integer and 7:0 as decimals.
pub type CR2R_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `CB2B` reader - Cb-to-Blue coefficient, with bits 9:8 as positive integer and 7:0 as decimals.
pub type CB2B_R = crate::FieldReader<u16>;
///Field `CB2B` writer - Cb-to-Blue coefficient, with bits 9:8 as positive integer and 7:0 as decimals.
pub type CB2B_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:9 - Cr-to-Red coefficient, with bits 9:8 as positive integer and 7:0 as decimals.
    #[inline(always)]
    pub fn cr2r(&self) -> CR2R_R {
        CR2R_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bits 16:25 - Cb-to-Blue coefficient, with bits 9:8 as positive integer and 7:0 as decimals.
    #[inline(always)]
    pub fn cb2b(&self) -> CB2B_R {
        CB2B_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2CYR0R")
            .field("cr2r", &self.cr2r())
            .field("cb2b", &self.cb2b())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - Cr-to-Red coefficient, with bits 9:8 as positive integer and 7:0 as decimals.
    #[inline(always)]
    pub fn cr2r(&mut self) -> CR2R_W<'_, L2CYR0Rrs> {
        CR2R_W::new(self, 0)
    }
    ///Bits 16:25 - Cb-to-Blue coefficient, with bits 9:8 as positive integer and 7:0 as decimals.
    #[inline(always)]
    pub fn cb2b(&mut self) -> CB2B_W<'_, L2CYR0Rrs> {
        CB2B_W::new(self, 16)
    }
}
/**LTDC layerx conversion YCbCr RGB 0 register

You can [`read`](crate::Reg::read) this register and get [`l2cyr0r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2cyr0r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#LTDC:L2CYR0R)*/
pub struct L2CYR0Rrs;
impl crate::RegisterSpec for L2CYR0Rrs {
    type Ux = u32;
}
///`read()` method returns [`l2cyr0r::R`](R) reader structure
impl crate::Readable for L2CYR0Rrs {}
///`write(|w| ..)` method takes [`l2cyr0r::W`](W) writer structure
impl crate::Writable for L2CYR0Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets L2CYR0R to value 0
impl crate::Resettable for L2CYR0Rrs {}
