///Register `ECCR2` reader
pub type R = crate::R<ECCR2rs>;
///Register `ECCR2` writer
pub type W = crate::W<ECCR2rs>;
///Field `ADDR_ECC` reader - ECC fail address
pub type ADDR_ECC_R = crate::FieldReader<u16>;
///Field `SYSF_ECC` reader - ECC fail for Corrected ECC Error or Double ECC Error in info block
pub type SYSF_ECC_R = crate::BitReader;
///Field `ECCIE` reader - ECC correction interrupt enable
pub type ECCIE_R = crate::BitReader;
///Field `ECCIE` writer - ECC correction interrupt enable
pub type ECCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ECCC` reader - ECC correction
pub type ECCC_R = crate::BitReader;
///Field `ECCC` writer - ECC correction
pub type ECCC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ECCD` reader - ECC detection
pub type ECCD_R = crate::BitReader;
///Field `ECCD` writer - ECC detection
pub type ECCD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:14 - ECC fail address
    #[inline(always)]
    pub fn addr_ecc(&self) -> ADDR_ECC_R {
        ADDR_ECC_R::new((self.bits & 0x7fff) as u16)
    }
    ///Bit 20 - ECC fail for Corrected ECC Error or Double ECC Error in info block
    #[inline(always)]
    pub fn sysf_ecc(&self) -> SYSF_ECC_R {
        SYSF_ECC_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 24 - ECC correction interrupt enable
    #[inline(always)]
    pub fn eccie(&self) -> ECCIE_R {
        ECCIE_R::new(((self.bits >> 24) & 1) != 0)
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
        f.debug_struct("ECCR2")
            .field("addr_ecc", &self.addr_ecc())
            .field("sysf_ecc", &self.sysf_ecc())
            .field("eccie", &self.eccie())
            .field("eccc", &self.eccc())
            .field("eccd", &self.eccd())
            .finish()
    }
}
impl W {
    ///Bit 24 - ECC correction interrupt enable
    #[inline(always)]
    pub fn eccie(&mut self) -> ECCIE_W<ECCR2rs> {
        ECCIE_W::new(self, 24)
    }
    ///Bit 30 - ECC correction
    #[inline(always)]
    pub fn eccc(&mut self) -> ECCC_W<ECCR2rs> {
        ECCC_W::new(self, 30)
    }
    ///Bit 31 - ECC detection
    #[inline(always)]
    pub fn eccd(&mut self) -> ECCD_W<ECCR2rs> {
        ECCD_W::new(self, 31)
    }
}
/**Flash ECC register 2

You can [`read`](crate::Reg::read) this register and get [`eccr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G031.html#FLASH:ECCR2)*/
pub struct ECCR2rs;
impl crate::RegisterSpec for ECCR2rs {
    type Ux = u32;
}
///`read()` method returns [`eccr2::R`](R) reader structure
impl crate::Readable for ECCR2rs {}
///`write(|w| ..)` method takes [`eccr2::W`](W) writer structure
impl crate::Writable for ECCR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ECCR2 to value 0
impl crate::Resettable for ECCR2rs {}
