///Register `CCCR` reader
pub type R = crate::R<CCCRrs>;
///Register `CCCR` writer
pub type W = crate::W<CCCRrs>;
///Field `INIT` reader - INIT
pub type INIT_R = crate::BitReader;
///Field `INIT` writer - INIT
pub type INIT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CCE` reader - CCE
pub type CCE_R = crate::BitReader;
///Field `CCE` writer - CCE
pub type CCE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ASM` reader - ASM
pub type ASM_R = crate::BitReader;
///Field `ASM` writer - ASM
pub type ASM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSA` reader - CSA
pub type CSA_R = crate::BitReader;
///Field `CSR` reader - CSR
pub type CSR_R = crate::BitReader;
///Field `CSR` writer - CSR
pub type CSR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MON` reader - MON
pub type MON_R = crate::BitReader;
///Field `MON` writer - MON
pub type MON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DAR` reader - DAR
pub type DAR_R = crate::BitReader;
///Field `DAR` writer - DAR
pub type DAR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEST` reader - TEST
pub type TEST_R = crate::BitReader;
///Field `TEST` writer - TEST
pub type TEST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FDOE` reader - FDOE
pub type FDOE_R = crate::BitReader;
///Field `FDOE` writer - FDOE
pub type FDOE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BRSE` reader - BRSE
pub type BRSE_R = crate::BitReader;
///Field `BRSE` writer - BRSE
pub type BRSE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PXHD` reader - PXHD
pub type PXHD_R = crate::BitReader;
///Field `PXHD` writer - PXHD
pub type PXHD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EFBI` reader - EFBI
pub type EFBI_R = crate::BitReader;
///Field `EFBI` writer - EFBI
pub type EFBI_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXP` reader - TXP
pub type TXP_R = crate::BitReader;
///Field `TXP` writer - TXP
pub type TXP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NISO` reader - NISO
pub type NISO_R = crate::BitReader;
///Field `NISO` writer - NISO
pub type NISO_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - INIT
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CCE
    #[inline(always)]
    pub fn cce(&self) -> CCE_R {
        CCE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ASM
    #[inline(always)]
    pub fn asm(&self) -> ASM_R {
        ASM_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - CSA
    #[inline(always)]
    pub fn csa(&self) -> CSA_R {
        CSA_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CSR
    #[inline(always)]
    pub fn csr(&self) -> CSR_R {
        CSR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - MON
    #[inline(always)]
    pub fn mon(&self) -> MON_R {
        MON_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - DAR
    #[inline(always)]
    pub fn dar(&self) -> DAR_R {
        DAR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - TEST
    #[inline(always)]
    pub fn test(&self) -> TEST_R {
        TEST_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - FDOE
    #[inline(always)]
    pub fn fdoe(&self) -> FDOE_R {
        FDOE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - BRSE
    #[inline(always)]
    pub fn brse(&self) -> BRSE_R {
        BRSE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 12 - PXHD
    #[inline(always)]
    pub fn pxhd(&self) -> PXHD_R {
        PXHD_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - EFBI
    #[inline(always)]
    pub fn efbi(&self) -> EFBI_R {
        EFBI_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - TXP
    #[inline(always)]
    pub fn txp(&self) -> TXP_R {
        TXP_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - NISO
    #[inline(always)]
    pub fn niso(&self) -> NISO_R {
        NISO_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCCR")
            .field("init", &self.init())
            .field("cce", &self.cce())
            .field("asm", &self.asm())
            .field("csa", &self.csa())
            .field("csr", &self.csr())
            .field("mon", &self.mon())
            .field("dar", &self.dar())
            .field("test", &self.test())
            .field("fdoe", &self.fdoe())
            .field("brse", &self.brse())
            .field("pxhd", &self.pxhd())
            .field("efbi", &self.efbi())
            .field("txp", &self.txp())
            .field("niso", &self.niso())
            .finish()
    }
}
impl W {
    ///Bit 0 - INIT
    #[inline(always)]
    pub fn init(&mut self) -> INIT_W<'_, CCCRrs> {
        INIT_W::new(self, 0)
    }
    ///Bit 1 - CCE
    #[inline(always)]
    pub fn cce(&mut self) -> CCE_W<'_, CCCRrs> {
        CCE_W::new(self, 1)
    }
    ///Bit 2 - ASM
    #[inline(always)]
    pub fn asm(&mut self) -> ASM_W<'_, CCCRrs> {
        ASM_W::new(self, 2)
    }
    ///Bit 4 - CSR
    #[inline(always)]
    pub fn csr(&mut self) -> CSR_W<'_, CCCRrs> {
        CSR_W::new(self, 4)
    }
    ///Bit 5 - MON
    #[inline(always)]
    pub fn mon(&mut self) -> MON_W<'_, CCCRrs> {
        MON_W::new(self, 5)
    }
    ///Bit 6 - DAR
    #[inline(always)]
    pub fn dar(&mut self) -> DAR_W<'_, CCCRrs> {
        DAR_W::new(self, 6)
    }
    ///Bit 7 - TEST
    #[inline(always)]
    pub fn test(&mut self) -> TEST_W<'_, CCCRrs> {
        TEST_W::new(self, 7)
    }
    ///Bit 8 - FDOE
    #[inline(always)]
    pub fn fdoe(&mut self) -> FDOE_W<'_, CCCRrs> {
        FDOE_W::new(self, 8)
    }
    ///Bit 9 - BRSE
    #[inline(always)]
    pub fn brse(&mut self) -> BRSE_W<'_, CCCRrs> {
        BRSE_W::new(self, 9)
    }
    ///Bit 12 - PXHD
    #[inline(always)]
    pub fn pxhd(&mut self) -> PXHD_W<'_, CCCRrs> {
        PXHD_W::new(self, 12)
    }
    ///Bit 13 - EFBI
    #[inline(always)]
    pub fn efbi(&mut self) -> EFBI_W<'_, CCCRrs> {
        EFBI_W::new(self, 13)
    }
    ///Bit 14 - TXP
    #[inline(always)]
    pub fn txp(&mut self) -> TXP_W<'_, CCCRrs> {
        TXP_W::new(self, 14)
    }
    ///Bit 15 - NISO
    #[inline(always)]
    pub fn niso(&mut self) -> NISO_W<'_, CCCRrs> {
        NISO_W::new(self, 15)
    }
}
/**For details about setting and resetting of single bits see Software initialization.

You can [`read`](crate::Reg::read) this register and get [`cccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:CCCR)*/
pub struct CCCRrs;
impl crate::RegisterSpec for CCCRrs {
    type Ux = u32;
}
///`read()` method returns [`cccr::R`](R) reader structure
impl crate::Readable for CCCRrs {}
///`write(|w| ..)` method takes [`cccr::W`](W) writer structure
impl crate::Writable for CCCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCCR to value 0x01
impl crate::Resettable for CCCRrs {
    const RESET_VALUE: u32 = 0x01;
}
