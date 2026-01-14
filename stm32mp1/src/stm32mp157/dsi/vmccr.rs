///Register `VMCCR` reader
pub type R = crate::R<VMCCRrs>;
///Field `VMT` reader - VMT
pub type VMT_R = crate::FieldReader;
///Field `LPVSAE` reader - LPVSAE
pub type LPVSAE_R = crate::BitReader;
///Field `LPVBPE` reader - LPVBPE
pub type LPVBPE_R = crate::BitReader;
///Field `LPVFPE` reader - LPVFPE
pub type LPVFPE_R = crate::BitReader;
///Field `LPVAE` reader - LPVAE
pub type LPVAE_R = crate::BitReader;
///Field `LPHBPE` reader - LPHBPE
pub type LPHBPE_R = crate::BitReader;
///Field `LPHFE` reader - LPHFE
pub type LPHFE_R = crate::BitReader;
///Field `FBTAAE` reader - FBTAAE
pub type FBTAAE_R = crate::BitReader;
///Field `LPCE` reader - LPCE
pub type LPCE_R = crate::BitReader;
impl R {
    ///Bits 0:1 - VMT
    #[inline(always)]
    pub fn vmt(&self) -> VMT_R {
        VMT_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - LPVSAE
    #[inline(always)]
    pub fn lpvsae(&self) -> LPVSAE_R {
        LPVSAE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - LPVBPE
    #[inline(always)]
    pub fn lpvbpe(&self) -> LPVBPE_R {
        LPVBPE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - LPVFPE
    #[inline(always)]
    pub fn lpvfpe(&self) -> LPVFPE_R {
        LPVFPE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - LPVAE
    #[inline(always)]
    pub fn lpvae(&self) -> LPVAE_R {
        LPVAE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - LPHBPE
    #[inline(always)]
    pub fn lphbpe(&self) -> LPHBPE_R {
        LPHBPE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - LPHFE
    #[inline(always)]
    pub fn lphfe(&self) -> LPHFE_R {
        LPHFE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - FBTAAE
    #[inline(always)]
    pub fn fbtaae(&self) -> FBTAAE_R {
        FBTAAE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - LPCE
    #[inline(always)]
    pub fn lpce(&self) -> LPCE_R {
        LPCE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VMCCR")
            .field("vmt", &self.vmt())
            .field("lpvsae", &self.lpvsae())
            .field("lpvbpe", &self.lpvbpe())
            .field("lpvfpe", &self.lpvfpe())
            .field("lpvae", &self.lpvae())
            .field("lphbpe", &self.lphbpe())
            .field("lphfe", &self.lphfe())
            .field("fbtaae", &self.fbtaae())
            .field("lpce", &self.lpce())
            .finish()
    }
}
/**DSI Host video mode current configuration register

You can [`read`](crate::Reg::read) this register and get [`vmccr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DSI:VMCCR)*/
pub struct VMCCRrs;
impl crate::RegisterSpec for VMCCRrs {
    type Ux = u32;
}
///`read()` method returns [`vmccr::R`](R) reader structure
impl crate::Readable for VMCCRrs {}
///`reset()` method sets VMCCR to value 0
impl crate::Resettable for VMCCRrs {}
