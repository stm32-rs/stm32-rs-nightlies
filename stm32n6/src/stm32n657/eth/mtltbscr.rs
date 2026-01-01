///Register `MTLTBSCR` reader
pub type R = crate::R<MTLTBSCRrs>;
///Register `MTLTBSCR` writer
pub type W = crate::W<MTLTBSCRrs>;
///Field `ESTM` reader - EST offset mode
pub type ESTM_R = crate::BitReader;
///Field `ESTM` writer - EST offset mode
pub type ESTM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LEOV` reader - Launch expiry offset valid
pub type LEOV_R = crate::BitReader;
///Field `LEOV` writer - Launch expiry offset valid
pub type LEOV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LEGOS` reader - Launch Expiry GSN Offset
pub type LEGOS_R = crate::FieldReader;
///Field `LEGOS` writer - Launch Expiry GSN Offset
pub type LEGOS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `LEOS` reader - Launch Expiry Offset
pub type LEOS_R = crate::FieldReader<u32>;
///Field `LEOS` writer - Launch Expiry Offset
pub type LEOS_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    ///Bit 0 - EST offset mode
    #[inline(always)]
    pub fn estm(&self) -> ESTM_R {
        ESTM_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Launch expiry offset valid
    #[inline(always)]
    pub fn leov(&self) -> LEOV_R {
        LEOV_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 4:6 - Launch Expiry GSN Offset
    #[inline(always)]
    pub fn legos(&self) -> LEGOS_R {
        LEGOS_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 8:31 - Launch Expiry Offset
    #[inline(always)]
    pub fn leos(&self) -> LEOS_R {
        LEOS_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MTLTBSCR")
            .field("estm", &self.estm())
            .field("leov", &self.leov())
            .field("legos", &self.legos())
            .field("leos", &self.leos())
            .finish()
    }
}
impl W {
    ///Bit 0 - EST offset mode
    #[inline(always)]
    pub fn estm(&mut self) -> ESTM_W<'_, MTLTBSCRrs> {
        ESTM_W::new(self, 0)
    }
    ///Bit 1 - Launch expiry offset valid
    #[inline(always)]
    pub fn leov(&mut self) -> LEOV_W<'_, MTLTBSCRrs> {
        LEOV_W::new(self, 1)
    }
    ///Bits 4:6 - Launch Expiry GSN Offset
    #[inline(always)]
    pub fn legos(&mut self) -> LEGOS_W<'_, MTLTBSCRrs> {
        LEGOS_W::new(self, 4)
    }
    ///Bits 8:31 - Launch Expiry Offset
    #[inline(always)]
    pub fn leos(&mut self) -> LEOS_W<'_, MTLTBSCRrs> {
        LEOS_W::new(self, 8)
    }
}
/**TBS control register

You can [`read`](crate::Reg::read) this register and get [`mtltbscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtltbscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ETH:MTLTBSCR)*/
pub struct MTLTBSCRrs;
impl crate::RegisterSpec for MTLTBSCRrs {
    type Ux = u32;
}
///`read()` method returns [`mtltbscr::R`](R) reader structure
impl crate::Readable for MTLTBSCRrs {}
///`write(|w| ..)` method takes [`mtltbscr::W`](W) writer structure
impl crate::Writable for MTLTBSCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MTLTBSCR to value 0
impl crate::Resettable for MTLTBSCRrs {}
