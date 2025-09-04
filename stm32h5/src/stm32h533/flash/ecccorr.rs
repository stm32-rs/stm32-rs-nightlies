///Register `ECCCORR` reader
pub type R = crate::R<ECCCORRrs>;
///Register `ECCCORR` writer
pub type W = crate::W<ECCCORRrs>;
///Field `ADDR_ECC` reader - ECC error address
pub type ADDR_ECC_R = crate::FieldReader<u16>;
///Field `OBK_ECC` reader - Single ECC error corrected in flash OB Keys storage area.
pub type OBK_ECC_R = crate::BitReader;
///Field `EDATA_ECC` reader - ECC fail for corrected ECC error in flash high-cycle data area
pub type EDATA_ECC_R = crate::BitReader;
///Field `BK_ECC` reader - ECC fail bank for corrected ECC error
pub type BK_ECC_R = crate::BitReader;
///Field `SYSF_ECC` reader - ECC fail for corrected ECC error in system flash memory
pub type SYSF_ECC_R = crate::BitReader;
///Field `OTP_ECC` reader - OTP ECC error bit
pub type OTP_ECC_R = crate::BitReader;
///Field `ECCCIE` reader - ECC single correction error interrupt enable bit
pub type ECCCIE_R = crate::BitReader;
///Field `ECCCIE` writer - ECC single correction error interrupt enable bit
pub type ECCCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ECCC` reader - ECC correction set by hardware when single ECC error has been detected and corrected.
pub type ECCC_R = crate::BitReader;
///Field `ECCC` writer - ECC correction set by hardware when single ECC error has been detected and corrected.
pub type ECCC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:15 - ECC error address
    #[inline(always)]
    pub fn addr_ecc(&self) -> ADDR_ECC_R {
        ADDR_ECC_R::new((self.bits & 0xffff) as u16)
    }
    ///Bit 20 - Single ECC error corrected in flash OB Keys storage area.
    #[inline(always)]
    pub fn obk_ecc(&self) -> OBK_ECC_R {
        OBK_ECC_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - ECC fail for corrected ECC error in flash high-cycle data area
    #[inline(always)]
    pub fn edata_ecc(&self) -> EDATA_ECC_R {
        EDATA_ECC_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - ECC fail bank for corrected ECC error
    #[inline(always)]
    pub fn bk_ecc(&self) -> BK_ECC_R {
        BK_ECC_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - ECC fail for corrected ECC error in system flash memory
    #[inline(always)]
    pub fn sysf_ecc(&self) -> SYSF_ECC_R {
        SYSF_ECC_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - OTP ECC error bit
    #[inline(always)]
    pub fn otp_ecc(&self) -> OTP_ECC_R {
        OTP_ECC_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - ECC single correction error interrupt enable bit
    #[inline(always)]
    pub fn ecccie(&self) -> ECCCIE_R {
        ECCCIE_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 30 - ECC correction set by hardware when single ECC error has been detected and corrected.
    #[inline(always)]
    pub fn eccc(&self) -> ECCC_R {
        ECCC_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECCCORR")
            .field("addr_ecc", &self.addr_ecc())
            .field("obk_ecc", &self.obk_ecc())
            .field("edata_ecc", &self.edata_ecc())
            .field("bk_ecc", &self.bk_ecc())
            .field("sysf_ecc", &self.sysf_ecc())
            .field("otp_ecc", &self.otp_ecc())
            .field("ecccie", &self.ecccie())
            .field("eccc", &self.eccc())
            .finish()
    }
}
impl W {
    ///Bit 25 - ECC single correction error interrupt enable bit
    #[inline(always)]
    pub fn ecccie(&mut self) -> ECCCIE_W<ECCCORRrs> {
        ECCCIE_W::new(self, 25)
    }
    ///Bit 30 - ECC correction set by hardware when single ECC error has been detected and corrected.
    #[inline(always)]
    pub fn eccc(&mut self) -> ECCC_W<ECCCORRrs> {
        ECCC_W::new(self, 30)
    }
}
/**FLASH ECC correction register

You can [`read`](crate::Reg::read) this register and get [`ecccorr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecccorr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#FLASH:ECCCORR)*/
pub struct ECCCORRrs;
impl crate::RegisterSpec for ECCCORRrs {
    type Ux = u32;
}
///`read()` method returns [`ecccorr::R`](R) reader structure
impl crate::Readable for ECCCORRrs {}
///`write(|w| ..)` method takes [`ecccorr::W`](W) writer structure
impl crate::Writable for ECCCORRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ECCCORR to value 0
impl crate::Resettable for ECCCORRrs {}
