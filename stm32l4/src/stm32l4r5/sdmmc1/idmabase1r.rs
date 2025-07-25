///Register `IDMABASE1R` reader
pub type R = crate::R<IDMABASE1Rrs>;
///Register `IDMABASE1R` writer
pub type W = crate::W<IDMABASE1Rrs>;
///Field `IDMABASE1` reader - Buffer 1 memory base address, shall be word aligned (bit \[1:0\] are always 0 and read only)
pub type IDMABASE1_R = crate::FieldReader<u32>;
///Field `IDMABASE1` writer - Buffer 1 memory base address, shall be word aligned (bit \[1:0\] are always 0 and read only)
pub type IDMABASE1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Buffer 1 memory base address, shall be word aligned (bit \[1:0\] are always 0 and read only)
    #[inline(always)]
    pub fn idmabase1(&self) -> IDMABASE1_R {
        IDMABASE1_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IDMABASE1R")
            .field("idmabase1", &self.idmabase1())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Buffer 1 memory base address, shall be word aligned (bit \[1:0\] are always 0 and read only)
    #[inline(always)]
    pub fn idmabase1(&mut self) -> IDMABASE1_W<IDMABASE1Rrs> {
        IDMABASE1_W::new(self, 0)
    }
}
/**IDMA buffer 0 base address register

You can [`read`](crate::Reg::read) this register and get [`idmabase1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idmabase1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#SDMMC1:IDMABASE1R)*/
pub struct IDMABASE1Rrs;
impl crate::RegisterSpec for IDMABASE1Rrs {
    type Ux = u32;
}
///`read()` method returns [`idmabase1r::R`](R) reader structure
impl crate::Readable for IDMABASE1Rrs {}
///`write(|w| ..)` method takes [`idmabase1r::W`](W) writer structure
impl crate::Writable for IDMABASE1Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IDMABASE1R to value 0
impl crate::Resettable for IDMABASE1Rrs {}
