///Register `CSR` reader
pub type R = crate::R<CSRrs>;
///Register `CSR` writer
pub type W = crate::W<CSRrs>;
///Field `GO` reader - PKA start processing command. Writing 0 has no effect Writing 1 starts the encryption engine
pub type GO_R = crate::BitReader;
///Field `GO` writer - PKA start processing command. Writing 0 has no effect Writing 1 starts the encryption engine
pub type GO_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `READY` reader - PKA readiness status. 0: The PKA is still computing 1: The PKA is ready to start a new calculation
pub type READY_R = crate::BitReader;
///Field `SFT_RST` reader - PKA software reset. Writing 0 clears the bit and releases the PKA block reset. Writing 1 resets the PKA block. The PKA RAM content is not changed.
pub type SFT_RST_R = crate::BitReader;
///Field `SFT_RST` writer - PKA software reset. Writing 0 clears the bit and releases the PKA block reset. Writing 1 resets the PKA block. The PKA RAM content is not changed.
pub type SFT_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - PKA start processing command. Writing 0 has no effect Writing 1 starts the encryption engine
    #[inline(always)]
    pub fn go(&self) -> GO_R {
        GO_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - PKA readiness status. 0: The PKA is still computing 1: The PKA is ready to start a new calculation
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 7 - PKA software reset. Writing 0 clears the bit and releases the PKA block reset. Writing 1 resets the PKA block. The PKA RAM content is not changed.
    #[inline(always)]
    pub fn sft_rst(&self) -> SFT_RST_R {
        SFT_RST_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR")
            .field("go", &self.go())
            .field("ready", &self.ready())
            .field("sft_rst", &self.sft_rst())
            .finish()
    }
}
impl W {
    ///Bit 0 - PKA start processing command. Writing 0 has no effect Writing 1 starts the encryption engine
    #[inline(always)]
    pub fn go(&mut self) -> GO_W<'_, CSRrs> {
        GO_W::new(self, 0)
    }
    ///Bit 7 - PKA software reset. Writing 0 clears the bit and releases the PKA block reset. Writing 1 resets the PKA block. The PKA RAM content is not changed.
    #[inline(always)]
    pub fn sft_rst(&mut self) -> SFT_RST_W<'_, CSRrs> {
        SFT_RST_W::new(self, 7)
    }
}
/**PKA_CSR register

You can [`read`](crate::Reg::read) this register and get [`csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#PKA:CSR)*/
pub struct CSRrs;
impl crate::RegisterSpec for CSRrs {
    type Ux = u32;
}
///`read()` method returns [`csr::R`](R) reader structure
impl crate::Readable for CSRrs {}
///`write(|w| ..)` method takes [`csr::W`](W) writer structure
impl crate::Writable for CSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR to value 0
impl crate::Resettable for CSRrs {}
