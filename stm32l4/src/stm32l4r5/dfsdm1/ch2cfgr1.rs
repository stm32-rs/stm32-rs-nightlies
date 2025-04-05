///Register `CH2CFGR1` reader
pub type R = crate::R<CH2CFGR1rs>;
///Register `CH2CFGR1` writer
pub type W = crate::W<CH2CFGR1rs>;
///Field `SITP` reader - SITP
pub type SITP_R = crate::FieldReader;
///Field `SITP` writer - SITP
pub type SITP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SPICKSEL` reader - SPICKSEL
pub type SPICKSEL_R = crate::FieldReader;
///Field `SPICKSEL` writer - SPICKSEL
pub type SPICKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SCDEN` reader - SCDEN
pub type SCDEN_R = crate::BitReader;
///Field `SCDEN` writer - SCDEN
pub type SCDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CKABEN` reader - CKABEN
pub type CKABEN_R = crate::BitReader;
///Field `CKABEN` writer - CKABEN
pub type CKABEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHEN` reader - CHEN
pub type CHEN_R = crate::BitReader;
///Field `CHEN` writer - CHEN
pub type CHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHINSEL` reader - CHINSEL
pub type CHINSEL_R = crate::BitReader;
///Field `CHINSEL` writer - CHINSEL
pub type CHINSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DATMPX` reader - DATMPX
pub type DATMPX_R = crate::FieldReader;
///Field `DATMPX` writer - DATMPX
pub type DATMPX_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DATPACK` reader - DATPACK
pub type DATPACK_R = crate::FieldReader;
///Field `DATPACK` writer - DATPACK
pub type DATPACK_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - SITP
    #[inline(always)]
    pub fn sitp(&self) -> SITP_R {
        SITP_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - SPICKSEL
    #[inline(always)]
    pub fn spicksel(&self) -> SPICKSEL_R {
        SPICKSEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bit 5 - SCDEN
    #[inline(always)]
    pub fn scden(&self) -> SCDEN_R {
        SCDEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - CKABEN
    #[inline(always)]
    pub fn ckaben(&self) -> CKABEN_R {
        CKABEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - CHEN
    #[inline(always)]
    pub fn chen(&self) -> CHEN_R {
        CHEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - CHINSEL
    #[inline(always)]
    pub fn chinsel(&self) -> CHINSEL_R {
        CHINSEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 12:13 - DATMPX
    #[inline(always)]
    pub fn datmpx(&self) -> DATMPX_R {
        DATMPX_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - DATPACK
    #[inline(always)]
    pub fn datpack(&self) -> DATPACK_R {
        DATPACK_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH2CFGR1")
            .field("datpack", &self.datpack())
            .field("datmpx", &self.datmpx())
            .field("chinsel", &self.chinsel())
            .field("chen", &self.chen())
            .field("ckaben", &self.ckaben())
            .field("scden", &self.scden())
            .field("spicksel", &self.spicksel())
            .field("sitp", &self.sitp())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - SITP
    #[inline(always)]
    pub fn sitp(&mut self) -> SITP_W<CH2CFGR1rs> {
        SITP_W::new(self, 0)
    }
    ///Bits 2:3 - SPICKSEL
    #[inline(always)]
    pub fn spicksel(&mut self) -> SPICKSEL_W<CH2CFGR1rs> {
        SPICKSEL_W::new(self, 2)
    }
    ///Bit 5 - SCDEN
    #[inline(always)]
    pub fn scden(&mut self) -> SCDEN_W<CH2CFGR1rs> {
        SCDEN_W::new(self, 5)
    }
    ///Bit 6 - CKABEN
    #[inline(always)]
    pub fn ckaben(&mut self) -> CKABEN_W<CH2CFGR1rs> {
        CKABEN_W::new(self, 6)
    }
    ///Bit 7 - CHEN
    #[inline(always)]
    pub fn chen(&mut self) -> CHEN_W<CH2CFGR1rs> {
        CHEN_W::new(self, 7)
    }
    ///Bit 8 - CHINSEL
    #[inline(always)]
    pub fn chinsel(&mut self) -> CHINSEL_W<CH2CFGR1rs> {
        CHINSEL_W::new(self, 8)
    }
    ///Bits 12:13 - DATMPX
    #[inline(always)]
    pub fn datmpx(&mut self) -> DATMPX_W<CH2CFGR1rs> {
        DATMPX_W::new(self, 12)
    }
    ///Bits 14:15 - DATPACK
    #[inline(always)]
    pub fn datpack(&mut self) -> DATPACK_W<CH2CFGR1rs> {
        DATPACK_W::new(self, 14)
    }
}
/**CH2CFGR1

You can [`read`](crate::Reg::read) this register and get [`ch2cfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2cfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#DFSDM1:CH2CFGR1)*/
pub struct CH2CFGR1rs;
impl crate::RegisterSpec for CH2CFGR1rs {
    type Ux = u32;
}
///`read()` method returns [`ch2cfgr1::R`](R) reader structure
impl crate::Readable for CH2CFGR1rs {}
///`write(|w| ..)` method takes [`ch2cfgr1::W`](W) writer structure
impl crate::Writable for CH2CFGR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CH2CFGR1 to value 0
impl crate::Resettable for CH2CFGR1rs {}
