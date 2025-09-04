///Register `C3CR` reader
pub type R = crate::R<C3CRrs>;
///Register `C3CR` writer
pub type W = crate::W<C3CRrs>;
///Field `DMAREQ_ID` reader - DMAREQ_ID
pub type DMAREQ_ID_R = crate::FieldReader;
///Field `DMAREQ_ID` writer - DMAREQ_ID
pub type DMAREQ_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `SOIE` reader - SOIE
pub type SOIE_R = crate::BitReader;
///Field `SOIE` writer - SOIE
pub type SOIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EGE` reader - EGE
pub type EGE_R = crate::BitReader;
///Field `EGE` writer - EGE
pub type EGE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SE` reader - SE
pub type SE_R = crate::BitReader;
///Field `SE` writer - SE
pub type SE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPOL` reader - SPOL
pub type SPOL_R = crate::FieldReader;
///Field `SPOL` writer - SPOL
pub type SPOL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `NBREQ` reader - NBREQ
pub type NBREQ_R = crate::FieldReader;
///Field `NBREQ` writer - NBREQ
pub type NBREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `SYNC_ID` reader - SYNC_ID
pub type SYNC_ID_R = crate::FieldReader;
///Field `SYNC_ID` writer - SYNC_ID
pub type SYNC_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:6 - DMAREQ_ID
    #[inline(always)]
    pub fn dmareq_id(&self) -> DMAREQ_ID_R {
        DMAREQ_ID_R::new((self.bits & 0x7f) as u8)
    }
    ///Bit 8 - SOIE
    #[inline(always)]
    pub fn soie(&self) -> SOIE_R {
        SOIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - EGE
    #[inline(always)]
    pub fn ege(&self) -> EGE_R {
        EGE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 16 - SE
    #[inline(always)]
    pub fn se(&self) -> SE_R {
        SE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:18 - SPOL
    #[inline(always)]
    pub fn spol(&self) -> SPOL_R {
        SPOL_R::new(((self.bits >> 17) & 3) as u8)
    }
    ///Bits 19:23 - NBREQ
    #[inline(always)]
    pub fn nbreq(&self) -> NBREQ_R {
        NBREQ_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    ///Bits 24:26 - SYNC_ID
    #[inline(always)]
    pub fn sync_id(&self) -> SYNC_ID_R {
        SYNC_ID_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C3CR")
            .field("dmareq_id", &self.dmareq_id())
            .field("soie", &self.soie())
            .field("ege", &self.ege())
            .field("se", &self.se())
            .field("spol", &self.spol())
            .field("nbreq", &self.nbreq())
            .field("sync_id", &self.sync_id())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - DMAREQ_ID
    #[inline(always)]
    pub fn dmareq_id(&mut self) -> DMAREQ_ID_W<C3CRrs> {
        DMAREQ_ID_W::new(self, 0)
    }
    ///Bit 8 - SOIE
    #[inline(always)]
    pub fn soie(&mut self) -> SOIE_W<C3CRrs> {
        SOIE_W::new(self, 8)
    }
    ///Bit 9 - EGE
    #[inline(always)]
    pub fn ege(&mut self) -> EGE_W<C3CRrs> {
        EGE_W::new(self, 9)
    }
    ///Bit 16 - SE
    #[inline(always)]
    pub fn se(&mut self) -> SE_W<C3CRrs> {
        SE_W::new(self, 16)
    }
    ///Bits 17:18 - SPOL
    #[inline(always)]
    pub fn spol(&mut self) -> SPOL_W<C3CRrs> {
        SPOL_W::new(self, 17)
    }
    ///Bits 19:23 - NBREQ
    #[inline(always)]
    pub fn nbreq(&mut self) -> NBREQ_W<C3CRrs> {
        NBREQ_W::new(self, 19)
    }
    ///Bits 24:26 - SYNC_ID
    #[inline(always)]
    pub fn sync_id(&mut self) -> SYNC_ID_W<C3CRrs> {
        SYNC_ID_W::new(self, 24)
    }
}
/**DMAMUX request line multiplexer channel 3 configuration register

You can [`read`](crate::Reg::read) this register and get [`c3cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DMAMUX1:C3CR)*/
pub struct C3CRrs;
impl crate::RegisterSpec for C3CRrs {
    type Ux = u32;
}
///`read()` method returns [`c3cr::R`](R) reader structure
impl crate::Readable for C3CRrs {}
///`write(|w| ..)` method takes [`c3cr::W`](W) writer structure
impl crate::Writable for C3CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C3CR to value 0
impl crate::Resettable for C3CRrs {}
