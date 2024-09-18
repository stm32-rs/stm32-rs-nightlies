///Register `SAI_BFRCR` reader
pub type R = crate::R<SAI_BFRCRrs>;
///Register `SAI_BFRCR` writer
pub type W = crate::W<SAI_BFRCRrs>;
///Field `FRL` reader - FRL
pub type FRL_R = crate::FieldReader;
///Field `FRL` writer - FRL
pub type FRL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `FSALL` reader - FSALL
pub type FSALL_R = crate::FieldReader;
///Field `FSALL` writer - FSALL
pub type FSALL_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `FSDEF` reader - FSDEF
pub type FSDEF_R = crate::BitReader;
///Field `FSPOL` reader - FSPOL
pub type FSPOL_R = crate::BitReader;
///Field `FSPOL` writer - FSPOL
pub type FSPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FSOFF` reader - FSOFF
pub type FSOFF_R = crate::BitReader;
///Field `FSOFF` writer - FSOFF
pub type FSOFF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - FRL
    #[inline(always)]
    pub fn frl(&self) -> FRL_R {
        FRL_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:14 - FSALL
    #[inline(always)]
    pub fn fsall(&self) -> FSALL_R {
        FSALL_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    ///Bit 16 - FSDEF
    #[inline(always)]
    pub fn fsdef(&self) -> FSDEF_R {
        FSDEF_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - FSPOL
    #[inline(always)]
    pub fn fspol(&self) -> FSPOL_R {
        FSPOL_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - FSOFF
    #[inline(always)]
    pub fn fsoff(&self) -> FSOFF_R {
        FSOFF_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAI_BFRCR")
            .field("frl", &self.frl())
            .field("fsall", &self.fsall())
            .field("fsdef", &self.fsdef())
            .field("fspol", &self.fspol())
            .field("fsoff", &self.fsoff())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - FRL
    #[inline(always)]
    #[must_use]
    pub fn frl(&mut self) -> FRL_W<SAI_BFRCRrs> {
        FRL_W::new(self, 0)
    }
    ///Bits 8:14 - FSALL
    #[inline(always)]
    #[must_use]
    pub fn fsall(&mut self) -> FSALL_W<SAI_BFRCRrs> {
        FSALL_W::new(self, 8)
    }
    ///Bit 17 - FSPOL
    #[inline(always)]
    #[must_use]
    pub fn fspol(&mut self) -> FSPOL_W<SAI_BFRCRrs> {
        FSPOL_W::new(self, 17)
    }
    ///Bit 18 - FSOFF
    #[inline(always)]
    #[must_use]
    pub fn fsoff(&mut self) -> FSOFF_W<SAI_BFRCRrs> {
        FSOFF_W::new(self, 18)
    }
}
/**This register has no meaning in and SPDIF audio protocol

You can [`read`](crate::Reg::read) this register and get [`sai_bfrcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sai_bfrcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#SAI1:SAI_BFRCR)*/
pub struct SAI_BFRCRrs;
impl crate::RegisterSpec for SAI_BFRCRrs {
    type Ux = u32;
}
///`read()` method returns [`sai_bfrcr::R`](R) reader structure
impl crate::Readable for SAI_BFRCRrs {}
///`write(|w| ..)` method takes [`sai_bfrcr::W`](W) writer structure
impl crate::Writable for SAI_BFRCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SAI_BFRCR to value 0x07
impl crate::Resettable for SAI_BFRCRrs {
    const RESET_VALUE: u32 = 0x07;
}
