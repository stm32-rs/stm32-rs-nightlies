///Register `CMD` reader
pub type R = crate::R<CMDrs>;
///Register `CMD` writer
pub type W = crate::W<CMDrs>;
///Field `CMDINDEX` reader - CMDINDEX
pub type CMDINDEX_R = crate::FieldReader;
///Field `CMDINDEX` writer - CMDINDEX
pub type CMDINDEX_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `WAITRESP` reader - WAITRESP
pub type WAITRESP_R = crate::FieldReader;
///Field `WAITRESP` writer - WAITRESP
pub type WAITRESP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `WAITINT` reader - WAITINT
pub type WAITINT_R = crate::BitReader;
///Field `WAITINT` writer - WAITINT
pub type WAITINT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WAITPEND` reader - WAITPEND
pub type WAITPEND_R = crate::BitReader;
///Field `WAITPEND` writer - WAITPEND
pub type WAITPEND_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CPSMEN` reader - CPSMEN
pub type CPSMEN_R = crate::BitReader;
///Field `CPSMEN` writer - CPSMEN
pub type CPSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDIOSuspend` reader - SDIOSuspend
pub type SDIOSUSPEND_R = crate::BitReader;
///Field `SDIOSuspend` writer - SDIOSuspend
pub type SDIOSUSPEND_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ENCMDcompl` reader - ENCMDcompl
pub type ENCMDCOMPL_R = crate::BitReader;
///Field `ENCMDcompl` writer - ENCMDcompl
pub type ENCMDCOMPL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `nIEN` reader - nIEN
pub type N_IEN_R = crate::BitReader;
///Field `nIEN` writer - nIEN
pub type N_IEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CE_ATACMD` reader - CE_ATACMD
pub type CE_ATACMD_R = crate::BitReader;
///Field `CE_ATACMD` writer - CE_ATACMD
pub type CE_ATACMD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:5 - CMDINDEX
    #[inline(always)]
    pub fn cmdindex(&self) -> CMDINDEX_R {
        CMDINDEX_R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 6:7 - WAITRESP
    #[inline(always)]
    pub fn waitresp(&self) -> WAITRESP_R {
        WAITRESP_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bit 8 - WAITINT
    #[inline(always)]
    pub fn waitint(&self) -> WAITINT_R {
        WAITINT_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - WAITPEND
    #[inline(always)]
    pub fn waitpend(&self) -> WAITPEND_R {
        WAITPEND_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - CPSMEN
    #[inline(always)]
    pub fn cpsmen(&self) -> CPSMEN_R {
        CPSMEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - SDIOSuspend
    #[inline(always)]
    pub fn sdiosuspend(&self) -> SDIOSUSPEND_R {
        SDIOSUSPEND_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - ENCMDcompl
    #[inline(always)]
    pub fn encmdcompl(&self) -> ENCMDCOMPL_R {
        ENCMDCOMPL_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - nIEN
    #[inline(always)]
    pub fn n_ien(&self) -> N_IEN_R {
        N_IEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - CE_ATACMD
    #[inline(always)]
    pub fn ce_atacmd(&self) -> CE_ATACMD_R {
        CE_ATACMD_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMD")
            .field("cmdindex", &self.cmdindex())
            .field("waitresp", &self.waitresp())
            .field("waitint", &self.waitint())
            .field("waitpend", &self.waitpend())
            .field("cpsmen", &self.cpsmen())
            .field("sdiosuspend", &self.sdiosuspend())
            .field("encmdcompl", &self.encmdcompl())
            .field("n_ien", &self.n_ien())
            .field("ce_atacmd", &self.ce_atacmd())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - CMDINDEX
    #[inline(always)]
    pub fn cmdindex(&mut self) -> CMDINDEX_W<CMDrs> {
        CMDINDEX_W::new(self, 0)
    }
    ///Bits 6:7 - WAITRESP
    #[inline(always)]
    pub fn waitresp(&mut self) -> WAITRESP_W<CMDrs> {
        WAITRESP_W::new(self, 6)
    }
    ///Bit 8 - WAITINT
    #[inline(always)]
    pub fn waitint(&mut self) -> WAITINT_W<CMDrs> {
        WAITINT_W::new(self, 8)
    }
    ///Bit 9 - WAITPEND
    #[inline(always)]
    pub fn waitpend(&mut self) -> WAITPEND_W<CMDrs> {
        WAITPEND_W::new(self, 9)
    }
    ///Bit 10 - CPSMEN
    #[inline(always)]
    pub fn cpsmen(&mut self) -> CPSMEN_W<CMDrs> {
        CPSMEN_W::new(self, 10)
    }
    ///Bit 11 - SDIOSuspend
    #[inline(always)]
    pub fn sdiosuspend(&mut self) -> SDIOSUSPEND_W<CMDrs> {
        SDIOSUSPEND_W::new(self, 11)
    }
    ///Bit 12 - ENCMDcompl
    #[inline(always)]
    pub fn encmdcompl(&mut self) -> ENCMDCOMPL_W<CMDrs> {
        ENCMDCOMPL_W::new(self, 12)
    }
    ///Bit 13 - nIEN
    #[inline(always)]
    pub fn n_ien(&mut self) -> N_IEN_W<CMDrs> {
        N_IEN_W::new(self, 13)
    }
    ///Bit 14 - CE_ATACMD
    #[inline(always)]
    pub fn ce_atacmd(&mut self) -> CE_ATACMD_W<CMDrs> {
        CE_ATACMD_W::new(self, 14)
    }
}
/**SDIO command register (SDIO_CMD)

You can [`read`](crate::Reg::read) this register and get [`cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F101.html#SDIO:CMD)*/
pub struct CMDrs;
impl crate::RegisterSpec for CMDrs {
    type Ux = u32;
}
///`read()` method returns [`cmd::R`](R) reader structure
impl crate::Readable for CMDrs {}
///`write(|w| ..)` method takes [`cmd::W`](W) writer structure
impl crate::Writable for CMDrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CMD to value 0
impl crate::Resettable for CMDrs {
    const RESET_VALUE: u32 = 0;
}
