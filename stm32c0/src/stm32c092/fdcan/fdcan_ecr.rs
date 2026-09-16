///Register `FDCAN_ECR` reader
pub type R = crate::R<FDCAN_ECRrs>;
///Register `FDCAN_ECR` writer
pub type W = crate::W<FDCAN_ECRrs>;
///Field `TEC` reader - Transmit error counter
pub type TEC_R = crate::FieldReader;
///Field `REC` reader - Receive error counter
pub type REC_R = crate::FieldReader;
/**Receive error passive

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RP {
    ///0: The receive error counter is below the error passive level of 128.
    B0x0 = 0,
    ///1: The receive error counter has reached the error passive level of 128.
    B0x1 = 1,
}
impl From<RP> for bool {
    #[inline(always)]
    fn from(variant: RP) -> Self {
        variant as u8 != 0
    }
}
///Field `RP` reader - Receive error passive
pub type RP_R = crate::BitReader<RP>;
impl RP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RP {
        match self.bits {
            false => RP::B0x0,
            true => RP::B0x1,
        }
    }
    ///The receive error counter is below the error passive level of 128.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RP::B0x0
    }
    ///The receive error counter has reached the error passive level of 128.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RP::B0x1
    }
}
///Field `CEL` reader - CAN error logging
pub type CEL_R = crate::FieldReader;
///Field `CEL` writer - CAN error logging
pub type CEL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Transmit error counter
    #[inline(always)]
    pub fn tec(&self) -> TEC_R {
        TEC_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:14 - Receive error counter
    #[inline(always)]
    pub fn rec(&self) -> REC_R {
        REC_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    ///Bit 15 - Receive error passive
    #[inline(always)]
    pub fn rp(&self) -> RP_R {
        RP_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:23 - CAN error logging
    #[inline(always)]
    pub fn cel(&self) -> CEL_R {
        CEL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_ECR")
            .field("tec", &self.tec())
            .field("rec", &self.rec())
            .field("rp", &self.rp())
            .field("cel", &self.cel())
            .finish()
    }
}
impl W {
    ///Bits 16:23 - CAN error logging
    #[inline(always)]
    pub fn cel(&mut self) -> CEL_W<'_, FDCAN_ECRrs> {
        CEL_W::new(self, 16)
    }
}
/**FDCAN error counter register

You can [`read`](crate::Reg::read) this register and get [`fdcan_ecr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ecr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#FDCAN:FDCAN_ECR)*/
pub struct FDCAN_ECRrs;
impl crate::RegisterSpec for FDCAN_ECRrs {
    type Ux = u32;
}
///`read()` method returns [`fdcan_ecr::R`](R) reader structure
impl crate::Readable for FDCAN_ECRrs {}
///`write(|w| ..)` method takes [`fdcan_ecr::W`](W) writer structure
impl crate::Writable for FDCAN_ECRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FDCAN_ECR to value 0
impl crate::Resettable for FDCAN_ECRrs {}
