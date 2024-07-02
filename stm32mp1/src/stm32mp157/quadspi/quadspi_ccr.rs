///Register `QUADSPI_CCR` reader
pub type R = crate::R<QUADSPI_CCRrs>;
///Register `QUADSPI_CCR` writer
pub type W = crate::W<QUADSPI_CCRrs>;
///Field `INSTRUCTION` reader - INSTRUCTION
pub type INSTRUCTION_R = crate::FieldReader;
///Field `INSTRUCTION` writer - INSTRUCTION
pub type INSTRUCTION_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `IMODE` reader - IMODE
pub type IMODE_R = crate::FieldReader;
///Field `IMODE` writer - IMODE
pub type IMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ADMODE` reader - ADMODE
pub type ADMODE_R = crate::FieldReader;
///Field `ADMODE` writer - ADMODE
pub type ADMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ADSIZE` reader - ADSIZE
pub type ADSIZE_R = crate::FieldReader;
///Field `ADSIZE` writer - ADSIZE
pub type ADSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ABMODE` reader - ABMODE
pub type ABMODE_R = crate::FieldReader;
///Field `ABMODE` writer - ABMODE
pub type ABMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ABSIZE` reader - ABSIZE
pub type ABSIZE_R = crate::FieldReader;
///Field `ABSIZE` writer - ABSIZE
pub type ABSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DCYC` reader - DCYC
pub type DCYC_R = crate::FieldReader;
///Field `DCYC` writer - DCYC
pub type DCYC_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `DMODE` reader - DMODE
pub type DMODE_R = crate::FieldReader;
///Field `DMODE` writer - DMODE
pub type DMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `FMODE` reader - FMODE
pub type FMODE_R = crate::FieldReader;
///Field `FMODE` writer - FMODE
pub type FMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SIOO` reader - SIOO
pub type SIOO_R = crate::BitReader;
///Field `SIOO` writer - SIOO
pub type SIOO_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FRCM` reader - FRCM
pub type FRCM_R = crate::BitReader;
///Field `FRCM` writer - FRCM
pub type FRCM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DHHC` reader - DHHC
pub type DHHC_R = crate::BitReader;
///Field `DHHC` writer - DHHC
pub type DHHC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DDRM` reader - DDRM
pub type DDRM_R = crate::BitReader;
///Field `DDRM` writer - DDRM
pub type DDRM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - INSTRUCTION
    #[inline(always)]
    pub fn instruction(&self) -> INSTRUCTION_R {
        INSTRUCTION_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:9 - IMODE
    #[inline(always)]
    pub fn imode(&self) -> IMODE_R {
        IMODE_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - ADMODE
    #[inline(always)]
    pub fn admode(&self) -> ADMODE_R {
        ADMODE_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - ADSIZE
    #[inline(always)]
    pub fn adsize(&self) -> ADSIZE_R {
        ADSIZE_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - ABMODE
    #[inline(always)]
    pub fn abmode(&self) -> ABMODE_R {
        ABMODE_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:17 - ABSIZE
    #[inline(always)]
    pub fn absize(&self) -> ABSIZE_R {
        ABSIZE_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:22 - DCYC
    #[inline(always)]
    pub fn dcyc(&self) -> DCYC_R {
        DCYC_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    ///Bits 24:25 - DMODE
    #[inline(always)]
    pub fn dmode(&self) -> DMODE_R {
        DMODE_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 26:27 - FMODE
    #[inline(always)]
    pub fn fmode(&self) -> FMODE_R {
        FMODE_R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bit 28 - SIOO
    #[inline(always)]
    pub fn sioo(&self) -> SIOO_R {
        SIOO_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - FRCM
    #[inline(always)]
    pub fn frcm(&self) -> FRCM_R {
        FRCM_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - DHHC
    #[inline(always)]
    pub fn dhhc(&self) -> DHHC_R {
        DHHC_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - DDRM
    #[inline(always)]
    pub fn ddrm(&self) -> DDRM_R {
        DDRM_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QUADSPI_CCR")
            .field("instruction", &self.instruction())
            .field("imode", &self.imode())
            .field("admode", &self.admode())
            .field("adsize", &self.adsize())
            .field("abmode", &self.abmode())
            .field("absize", &self.absize())
            .field("dcyc", &self.dcyc())
            .field("dmode", &self.dmode())
            .field("fmode", &self.fmode())
            .field("sioo", &self.sioo())
            .field("frcm", &self.frcm())
            .field("dhhc", &self.dhhc())
            .field("ddrm", &self.ddrm())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - INSTRUCTION
    #[inline(always)]
    #[must_use]
    pub fn instruction(&mut self) -> INSTRUCTION_W<QUADSPI_CCRrs> {
        INSTRUCTION_W::new(self, 0)
    }
    ///Bits 8:9 - IMODE
    #[inline(always)]
    #[must_use]
    pub fn imode(&mut self) -> IMODE_W<QUADSPI_CCRrs> {
        IMODE_W::new(self, 8)
    }
    ///Bits 10:11 - ADMODE
    #[inline(always)]
    #[must_use]
    pub fn admode(&mut self) -> ADMODE_W<QUADSPI_CCRrs> {
        ADMODE_W::new(self, 10)
    }
    ///Bits 12:13 - ADSIZE
    #[inline(always)]
    #[must_use]
    pub fn adsize(&mut self) -> ADSIZE_W<QUADSPI_CCRrs> {
        ADSIZE_W::new(self, 12)
    }
    ///Bits 14:15 - ABMODE
    #[inline(always)]
    #[must_use]
    pub fn abmode(&mut self) -> ABMODE_W<QUADSPI_CCRrs> {
        ABMODE_W::new(self, 14)
    }
    ///Bits 16:17 - ABSIZE
    #[inline(always)]
    #[must_use]
    pub fn absize(&mut self) -> ABSIZE_W<QUADSPI_CCRrs> {
        ABSIZE_W::new(self, 16)
    }
    ///Bits 18:22 - DCYC
    #[inline(always)]
    #[must_use]
    pub fn dcyc(&mut self) -> DCYC_W<QUADSPI_CCRrs> {
        DCYC_W::new(self, 18)
    }
    ///Bits 24:25 - DMODE
    #[inline(always)]
    #[must_use]
    pub fn dmode(&mut self) -> DMODE_W<QUADSPI_CCRrs> {
        DMODE_W::new(self, 24)
    }
    ///Bits 26:27 - FMODE
    #[inline(always)]
    #[must_use]
    pub fn fmode(&mut self) -> FMODE_W<QUADSPI_CCRrs> {
        FMODE_W::new(self, 26)
    }
    ///Bit 28 - SIOO
    #[inline(always)]
    #[must_use]
    pub fn sioo(&mut self) -> SIOO_W<QUADSPI_CCRrs> {
        SIOO_W::new(self, 28)
    }
    ///Bit 29 - FRCM
    #[inline(always)]
    #[must_use]
    pub fn frcm(&mut self) -> FRCM_W<QUADSPI_CCRrs> {
        FRCM_W::new(self, 29)
    }
    ///Bit 30 - DHHC
    #[inline(always)]
    #[must_use]
    pub fn dhhc(&mut self) -> DHHC_W<QUADSPI_CCRrs> {
        DHHC_W::new(self, 30)
    }
    ///Bit 31 - DDRM
    #[inline(always)]
    #[must_use]
    pub fn ddrm(&mut self) -> DDRM_W<QUADSPI_CCRrs> {
        DDRM_W::new(self, 31)
    }
}
/**QUADSPI communication configuration register

You can [`read`](crate::Reg::read) this register and get [`quadspi_ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`quadspi_ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#QUADSPI:QUADSPI_CCR)*/
pub struct QUADSPI_CCRrs;
impl crate::RegisterSpec for QUADSPI_CCRrs {
    type Ux = u32;
}
///`read()` method returns [`quadspi_ccr::R`](R) reader structure
impl crate::Readable for QUADSPI_CCRrs {}
///`write(|w| ..)` method takes [`quadspi_ccr::W`](W) writer structure
impl crate::Writable for QUADSPI_CCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets QUADSPI_CCR to value 0
impl crate::Resettable for QUADSPI_CCRrs {
    const RESET_VALUE: u32 = 0;
}
