///Register `IDMABASE0R` reader
pub type R = crate::R<IDMABASE0Rrs>;
///Register `IDMABASE0R` writer
pub type W = crate::W<IDMABASE0Rrs>;
///Field `IDMABASE0` reader - Buffer 0 memory base address bits \[31:2\], shall be word aligned (bit \[1:0\] are always 0 and read only). This register can be written by firmware when DPSM is inactive (DPSMACT = 0), and can dynamically be written by firmware when DPSM active (DPSMACT = 1) and memory buffer 0 is inactive (IDMABACT = 1).
pub type IDMABASE0_R = crate::FieldReader<u32>;
///Field `IDMABASE0` writer - Buffer 0 memory base address bits \[31:2\], shall be word aligned (bit \[1:0\] are always 0 and read only). This register can be written by firmware when DPSM is inactive (DPSMACT = 0), and can dynamically be written by firmware when DPSM active (DPSMACT = 1) and memory buffer 0 is inactive (IDMABACT = 1).
pub type IDMABASE0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Buffer 0 memory base address bits \[31:2\], shall be word aligned (bit \[1:0\] are always 0 and read only). This register can be written by firmware when DPSM is inactive (DPSMACT = 0), and can dynamically be written by firmware when DPSM active (DPSMACT = 1) and memory buffer 0 is inactive (IDMABACT = 1).
    #[inline(always)]
    pub fn idmabase0(&self) -> IDMABASE0_R {
        IDMABASE0_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IDMABASE0R")
            .field("idmabase0", &self.idmabase0())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Buffer 0 memory base address bits \[31:2\], shall be word aligned (bit \[1:0\] are always 0 and read only). This register can be written by firmware when DPSM is inactive (DPSMACT = 0), and can dynamically be written by firmware when DPSM active (DPSMACT = 1) and memory buffer 0 is inactive (IDMABACT = 1).
    #[inline(always)]
    pub fn idmabase0(&mut self) -> IDMABASE0_W<'_, IDMABASE0Rrs> {
        IDMABASE0_W::new(self, 0)
    }
}
/**The SDMMC_IDMABASE0R register contains the memory buffer base address in single buffer configuration and the buffer 0 base address in double buffer configuration.

You can [`read`](crate::Reg::read) this register and get [`idmabase0r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idmabase0r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM4.html#SDMMC1:IDMABASE0R)*/
pub struct IDMABASE0Rrs;
impl crate::RegisterSpec for IDMABASE0Rrs {
    type Ux = u32;
}
///`read()` method returns [`idmabase0r::R`](R) reader structure
impl crate::Readable for IDMABASE0Rrs {}
///`write(|w| ..)` method takes [`idmabase0r::W`](W) writer structure
impl crate::Writable for IDMABASE0Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IDMABASE0R to value 0
impl crate::Resettable for IDMABASE0Rrs {}
