///Register `ECCR` reader
pub type R = crate::R<ECCRrs>;
///Register `ECCR` writer
pub type W = crate::W<ECCRrs>;
///Field `ADDR_ECC` reader - ECC fail address
pub type ADDR_ECC_R = crate::FieldReader<u32>;
///Field `BK_ECC` reader - ECC fail bank
pub type BK_ECC_R = crate::BitReader;
///Field `SYSF_ECC` reader - System Flash ECC fail
pub type SYSF_ECC_R = crate::BitReader;
///Field `ECCCIE` reader - ECC correction interrupt enable
pub type ECCCIE_R = crate::BitReader;
///Field `ECCCIE` writer - ECC correction interrupt enable
pub type ECCCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ECCC2` reader - ECC correction
pub type ECCC2_R = crate::BitReader;
///Field `ECCC2` writer - ECC correction
pub type ECCC2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ECCD2` reader - ECC2 detection
pub type ECCD2_R = crate::BitReader;
///Field `ECCD2` writer - ECC2 detection
pub type ECCD2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ECCC` reader - ECC correction
pub type ECCC_R = crate::BitReader;
///Field `ECCC` writer - ECC correction
pub type ECCC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ECCD` reader - ECC detection
pub type ECCD_R = crate::BitReader;
///Field `ECCD` writer - ECC detection
pub type ECCD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:18 - ECC fail address
    #[inline(always)]
    pub fn addr_ecc(&self) -> ADDR_ECC_R {
        ADDR_ECC_R::new(self.bits & 0x0007_ffff)
    }
    ///Bit 21 - ECC fail bank
    #[inline(always)]
    pub fn bk_ecc(&self) -> BK_ECC_R {
        BK_ECC_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - System Flash ECC fail
    #[inline(always)]
    pub fn sysf_ecc(&self) -> SYSF_ECC_R {
        SYSF_ECC_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 24 - ECC correction interrupt enable
    #[inline(always)]
    pub fn ecccie(&self) -> ECCCIE_R {
        ECCCIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 28 - ECC correction
    #[inline(always)]
    pub fn eccc2(&self) -> ECCC2_R {
        ECCC2_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - ECC2 detection
    #[inline(always)]
    pub fn eccd2(&self) -> ECCD2_R {
        ECCD2_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - ECC correction
    #[inline(always)]
    pub fn eccc(&self) -> ECCC_R {
        ECCC_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - ECC detection
    #[inline(always)]
    pub fn eccd(&self) -> ECCD_R {
        ECCD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECCR")
            .field("addr_ecc", &self.addr_ecc())
            .field("bk_ecc", &self.bk_ecc())
            .field("sysf_ecc", &self.sysf_ecc())
            .field("ecccie", &self.ecccie())
            .field("eccc2", &self.eccc2())
            .field("eccd2", &self.eccd2())
            .field("eccc", &self.eccc())
            .field("eccd", &self.eccd())
            .finish()
    }
}
impl W {
    ///Bit 24 - ECC correction interrupt enable
    #[inline(always)]
    pub fn ecccie(&mut self) -> ECCCIE_W<'_, ECCRrs> {
        ECCCIE_W::new(self, 24)
    }
    ///Bit 28 - ECC correction
    #[inline(always)]
    pub fn eccc2(&mut self) -> ECCC2_W<'_, ECCRrs> {
        ECCC2_W::new(self, 28)
    }
    ///Bit 29 - ECC2 detection
    #[inline(always)]
    pub fn eccd2(&mut self) -> ECCD2_W<'_, ECCRrs> {
        ECCD2_W::new(self, 29)
    }
    ///Bit 30 - ECC correction
    #[inline(always)]
    pub fn eccc(&mut self) -> ECCC_W<'_, ECCRrs> {
        ECCC_W::new(self, 30)
    }
    ///Bit 31 - ECC detection
    #[inline(always)]
    pub fn eccd(&mut self) -> ECCD_W<'_, ECCRrs> {
        ECCD_W::new(self, 31)
    }
}
/**Flash ECC register

You can [`read`](crate::Reg::read) this register and get [`eccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474.html#FLASH:ECCR)*/
pub struct ECCRrs;
impl crate::RegisterSpec for ECCRrs {
    type Ux = u32;
}
///`read()` method returns [`eccr::R`](R) reader structure
impl crate::Readable for ECCRrs {}
///`write(|w| ..)` method takes [`eccr::W`](W) writer structure
impl crate::Writable for ECCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ECCR to value 0
impl crate::Resettable for ECCRrs {}
