///Register `SDMMC_IDMABASER` reader
pub type R = crate::R<SDMMC_IDMABASERrs>;
///Register `SDMMC_IDMABASER` writer
pub type W = crate::W<SDMMC_IDMABASERrs>;
/**Field `IDMABASE` reader - Buffer memory base address bits \[31:2\], shall be word aligned (bit \[1:0\]
are always 0 and read only)*/
pub type IDMABASE_R = crate::FieldReader<u32>;
/**Field `IDMABASE` writer - Buffer memory base address bits \[31:2\], shall be word aligned (bit \[1:0\]
are always 0 and read only)*/
pub type IDMABASE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    /**Bits 0:31 - Buffer memory base address bits \[31:2\], shall be word aligned (bit \[1:0\]
    are always 0 and read only)*/
    #[inline(always)]
    pub fn idmabase(&self) -> IDMABASE_R {
        IDMABASE_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDMMC_IDMABASER")
            .field("idmabase", &self.idmabase())
            .finish()
    }
}
impl W {
    /**Bits 0:31 - Buffer memory base address bits \[31:2\], shall be word aligned (bit \[1:0\]
    are always 0 and read only)*/
    #[inline(always)]
    #[must_use]
    pub fn idmabase(&mut self) -> IDMABASE_W<SDMMC_IDMABASERrs> {
        IDMABASE_W::new(self, 0)
    }
}
/**buffer base address register

You can [`read`](crate::Reg::read) this register and get [`sdmmc_idmabaser::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmmc_idmabaser::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#SDMMC:SDMMC_IDMABASER)*/
pub struct SDMMC_IDMABASERrs;
impl crate::RegisterSpec for SDMMC_IDMABASERrs {
    type Ux = u32;
}
///`read()` method returns [`sdmmc_idmabaser::R`](R) reader structure
impl crate::Readable for SDMMC_IDMABASERrs {}
///`write(|w| ..)` method takes [`sdmmc_idmabaser::W`](W) writer structure
impl crate::Writable for SDMMC_IDMABASERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SDMMC_IDMABASER to value 0
impl crate::Resettable for SDMMC_IDMABASERrs {
    const RESET_VALUE: u32 = 0;
}
