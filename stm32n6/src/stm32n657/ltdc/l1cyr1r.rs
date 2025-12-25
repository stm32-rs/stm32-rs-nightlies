///Register `L1CYR1R` reader
pub type R = crate::R<L1CYR1Rrs>;
///Register `L1CYR1R` writer
pub type W = crate::W<L1CYR1Rrs>;
///Field `CR2G` reader - Cr-to-Green coefficient, with bits 9:8 as positive integer and 7:0 as decimals.
pub type CR2G_R = crate::FieldReader<u16>;
///Field `CR2G` writer - Cr-to-Green coefficient, with bits 9:8 as positive integer and 7:0 as decimals.
pub type CR2G_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `CB2G` reader - Cb-to-Green coefficient, with bits 9:8 as positive integer and 7:0 as decimals.
pub type CB2G_R = crate::FieldReader<u16>;
///Field `CB2G` writer - Cb-to-Green coefficient, with bits 9:8 as positive integer and 7:0 as decimals.
pub type CB2G_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:9 - Cr-to-Green coefficient, with bits 9:8 as positive integer and 7:0 as decimals.
    #[inline(always)]
    pub fn cr2g(&self) -> CR2G_R {
        CR2G_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bits 16:25 - Cb-to-Green coefficient, with bits 9:8 as positive integer and 7:0 as decimals.
    #[inline(always)]
    pub fn cb2g(&self) -> CB2G_R {
        CB2G_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1CYR1R")
            .field("cr2g", &self.cr2g())
            .field("cb2g", &self.cb2g())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - Cr-to-Green coefficient, with bits 9:8 as positive integer and 7:0 as decimals.
    #[inline(always)]
    pub fn cr2g(&mut self) -> CR2G_W<'_, L1CYR1Rrs> {
        CR2G_W::new(self, 0)
    }
    ///Bits 16:25 - Cb-to-Green coefficient, with bits 9:8 as positive integer and 7:0 as decimals.
    #[inline(always)]
    pub fn cb2g(&mut self) -> CB2G_W<'_, L1CYR1Rrs> {
        CB2G_W::new(self, 16)
    }
}
/**LTDC layerx conversion YCbCr RGB 1 register

You can [`read`](crate::Reg::read) this register and get [`l1cyr1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1cyr1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#LTDC:L1CYR1R)*/
pub struct L1CYR1Rrs;
impl crate::RegisterSpec for L1CYR1Rrs {
    type Ux = u32;
}
///`read()` method returns [`l1cyr1r::R`](R) reader structure
impl crate::Readable for L1CYR1Rrs {}
///`write(|w| ..)` method takes [`l1cyr1r::W`](W) writer structure
impl crate::Writable for L1CYR1Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets L1CYR1R to value 0
impl crate::Resettable for L1CYR1Rrs {}
