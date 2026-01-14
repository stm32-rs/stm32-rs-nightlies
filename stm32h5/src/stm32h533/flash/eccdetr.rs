///Register `ECCDETR` reader
pub type R = crate::R<ECCDETRrs>;
///Register `ECCDETR` writer
pub type W = crate::W<ECCDETRrs>;
///Field `ADDR_ECC` reader - ECC error address
pub type ADDR_ECC_R = crate::FieldReader<u16>;
///Field `OBK_ECC` reader - ECC fail double ECC error in flash OB Keys storage area.
pub type OBK_ECC_R = crate::BitReader;
///Field `EDATA_ECC` reader - ECC fail double ECC error in flash high-cycle data area
pub type EDATA_ECC_R = crate::BitReader;
///Field `BK_ECC` reader - ECC fail bank for double ECC error
pub type BK_ECC_R = crate::BitReader;
///Field `SYSF_ECC` reader - ECC fail for double ECC error in system flash memory
pub type SYSF_ECC_R = crate::BitReader;
///Field `OTP_ECC` reader - OTP ECC error bit
pub type OTP_ECC_R = crate::BitReader;
///Field `ECCD` reader - ECC detection
pub type ECCD_R = crate::BitReader;
///Field `ECCD` writer - ECC detection
pub type ECCD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:15 - ECC error address
    #[inline(always)]
    pub fn addr_ecc(&self) -> ADDR_ECC_R {
        ADDR_ECC_R::new((self.bits & 0xffff) as u16)
    }
    ///Bit 20 - ECC fail double ECC error in flash OB Keys storage area.
    #[inline(always)]
    pub fn obk_ecc(&self) -> OBK_ECC_R {
        OBK_ECC_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - ECC fail double ECC error in flash high-cycle data area
    #[inline(always)]
    pub fn edata_ecc(&self) -> EDATA_ECC_R {
        EDATA_ECC_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - ECC fail bank for double ECC error
    #[inline(always)]
    pub fn bk_ecc(&self) -> BK_ECC_R {
        BK_ECC_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - ECC fail for double ECC error in system flash memory
    #[inline(always)]
    pub fn sysf_ecc(&self) -> SYSF_ECC_R {
        SYSF_ECC_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - OTP ECC error bit
    #[inline(always)]
    pub fn otp_ecc(&self) -> OTP_ECC_R {
        OTP_ECC_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 31 - ECC detection
    #[inline(always)]
    pub fn eccd(&self) -> ECCD_R {
        ECCD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECCDETR")
            .field("addr_ecc", &self.addr_ecc())
            .field("obk_ecc", &self.obk_ecc())
            .field("edata_ecc", &self.edata_ecc())
            .field("bk_ecc", &self.bk_ecc())
            .field("sysf_ecc", &self.sysf_ecc())
            .field("otp_ecc", &self.otp_ecc())
            .field("eccd", &self.eccd())
            .finish()
    }
}
impl W {
    ///Bit 31 - ECC detection
    #[inline(always)]
    pub fn eccd(&mut self) -> ECCD_W<'_, ECCDETRrs> {
        ECCD_W::new(self, 31)
    }
}
/**FLASH ECC detection register

You can [`read`](crate::Reg::read) this register and get [`eccdetr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccdetr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#FLASH:ECCDETR)*/
pub struct ECCDETRrs;
impl crate::RegisterSpec for ECCDETRrs {
    type Ux = u32;
}
///`read()` method returns [`eccdetr::R`](R) reader structure
impl crate::Readable for ECCDETRrs {}
///`write(|w| ..)` method takes [`eccdetr::W`](W) writer structure
impl crate::Writable for ECCDETRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ECCDETR to value 0
impl crate::Resettable for ECCDETRrs {}
