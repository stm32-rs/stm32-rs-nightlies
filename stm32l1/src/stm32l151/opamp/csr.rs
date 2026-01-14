///Register `CSR` reader
pub type R = crate::R<CSRrs>;
///Register `CSR` writer
pub type W = crate::W<CSRrs>;
///Field `OPA1PD` reader - OPAMP1 power down
pub type OPA1PD_R = crate::BitReader;
///Field `OPA1PD` writer - OPAMP1 power down
pub type OPA1PD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `S3SEL1` reader - Switch 3 for OPAMP1 enable
pub type S3SEL1_R = crate::BitReader;
///Field `S3SEL1` writer - Switch 3 for OPAMP1 enable
pub type S3SEL1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `S4SEL1` reader - Switch 4 for OPAMP1 enable
pub type S4SEL1_R = crate::BitReader;
///Field `S4SEL1` writer - Switch 4 for OPAMP1 enable
pub type S4SEL1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `S5SEL1` reader - Switch 5 for OPAMP1 enable
pub type S5SEL1_R = crate::BitReader;
///Field `S5SEL1` writer - Switch 5 for OPAMP1 enable
pub type S5SEL1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `S6SEL1` reader - Switch 6 for OPAMP1 enable
pub type S6SEL1_R = crate::BitReader;
///Field `S6SEL1` writer - Switch 6 for OPAMP1 enable
pub type S6SEL1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPA1CAL_L` reader - OPAMP1 offset calibration for P differential pair
pub type OPA1CAL_L_R = crate::BitReader;
///Field `OPA1CAL_L` writer - OPAMP1 offset calibration for P differential pair
pub type OPA1CAL_L_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPA1CAL_H` reader - OPAMP1 offset calibration for N differential pair
pub type OPA1CAL_H_R = crate::BitReader;
///Field `OPA1CAL_H` writer - OPAMP1 offset calibration for N differential pair
pub type OPA1CAL_H_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPA1LPM` reader - OPAMP1 low power mode
pub type OPA1LPM_R = crate::BitReader;
///Field `OPA1LPM` writer - OPAMP1 low power mode
pub type OPA1LPM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPA2PD` reader - OPAMP2 power down
pub type OPA2PD_R = crate::BitReader;
///Field `OPA2PD` writer - OPAMP2 power down
pub type OPA2PD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `S3SEL2` reader - Switch 3 for OPAMP2 enable
pub type S3SEL2_R = crate::BitReader;
///Field `S3SEL2` writer - Switch 3 for OPAMP2 enable
pub type S3SEL2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `S4SEL2` reader - Switch 4 for OPAMP2 enable
pub type S4SEL2_R = crate::BitReader;
///Field `S4SEL2` writer - Switch 4 for OPAMP2 enable
pub type S4SEL2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `S5SEL2` reader - Switch 5 for OPAMP2 enable
pub type S5SEL2_R = crate::BitReader;
///Field `S5SEL2` writer - Switch 5 for OPAMP2 enable
pub type S5SEL2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `S6SEL2` reader - Switch 6 for OPAMP2 enable
pub type S6SEL2_R = crate::BitReader;
///Field `S6SEL2` writer - Switch 6 for OPAMP2 enable
pub type S6SEL2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPA2CAL_L` reader - OPAMP2 offset Calibration for P differential pair
pub type OPA2CAL_L_R = crate::BitReader;
///Field `OPA2CAL_L` writer - OPAMP2 offset Calibration for P differential pair
pub type OPA2CAL_L_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPA2CAL_H` reader - OPAMP2 offset calibration for N differential pair
pub type OPA2CAL_H_R = crate::BitReader;
///Field `OPA2CAL_H` writer - OPAMP2 offset calibration for N differential pair
pub type OPA2CAL_H_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPA2LPM` reader - OPAMP2 low power mode
pub type OPA2LPM_R = crate::BitReader;
///Field `OPA2LPM` writer - OPAMP2 low power mode
pub type OPA2LPM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPA3PD` reader - OPAMP3 power down
pub type OPA3PD_R = crate::BitReader;
///Field `OPA3PD` writer - OPAMP3 power down
pub type OPA3PD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `S3SEL3` reader - Switch 3 for OPAMP3 Enable
pub type S3SEL3_R = crate::BitReader;
///Field `S3SEL3` writer - Switch 3 for OPAMP3 Enable
pub type S3SEL3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `S4SEL3` reader - Switch 4 for OPAMP3 enable
pub type S4SEL3_R = crate::BitReader;
///Field `S4SEL3` writer - Switch 4 for OPAMP3 enable
pub type S4SEL3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `S5SEL3` reader - Switch 5 for OPAMP3 enable
pub type S5SEL3_R = crate::BitReader;
///Field `S5SEL3` writer - Switch 5 for OPAMP3 enable
pub type S5SEL3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `S6SEL3` reader - Switch 6 for OPAMP3 enable
pub type S6SEL3_R = crate::BitReader;
///Field `S6SEL3` writer - Switch 6 for OPAMP3 enable
pub type S6SEL3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPA3CAL_L` reader - OPAMP3 offset Calibration for P differential pair
pub type OPA3CAL_L_R = crate::BitReader;
///Field `OPA3CAL_L` writer - OPAMP3 offset Calibration for P differential pair
pub type OPA3CAL_L_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPA3CAL_H` reader - OPAMP3 offset calibration for N differential pair
pub type OPA3CAL_H_R = crate::BitReader;
///Field `OPA3CAL_H` writer - OPAMP3 offset calibration for N differential pair
pub type OPA3CAL_H_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPA3LPM` reader - OPAMP3 low power mode
pub type OPA3LPM_R = crate::BitReader;
///Field `OPA3LPM` writer - OPAMP3 low power mode
pub type OPA3LPM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ANAWSEL1` reader - Switch SanA enable for OPAMP1
pub type ANAWSEL1_R = crate::BitReader;
///Field `ANAWSEL1` writer - Switch SanA enable for OPAMP1
pub type ANAWSEL1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ANAWSEL2` reader - Switch SanA enable for OPAMP2
pub type ANAWSEL2_R = crate::BitReader;
///Field `ANAWSEL2` writer - Switch SanA enable for OPAMP2
pub type ANAWSEL2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ANAWSEL3` reader - Switch SanA enable for OPAMP3
pub type ANAWSEL3_R = crate::BitReader;
///Field `ANAWSEL3` writer - Switch SanA enable for OPAMP3
pub type ANAWSEL3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `S7SEL2` reader - Switch 7 for OPAMP2 enable
pub type S7SEL2_R = crate::BitReader;
///Field `S7SEL2` writer - Switch 7 for OPAMP2 enable
pub type S7SEL2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AOP_RANGE` reader - Power range selection
pub type AOP_RANGE_R = crate::BitReader;
///Field `AOP_RANGE` writer - Power range selection
pub type AOP_RANGE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPA1CALOUT` reader - OPAMP1 calibration output
pub type OPA1CALOUT_R = crate::BitReader;
///Field `OPA1CALOUT` writer - OPAMP1 calibration output
pub type OPA1CALOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPA2CALOUT` reader - OPAMP2 calibration output
pub type OPA2CALOUT_R = crate::BitReader;
///Field `OPA2CALOUT` writer - OPAMP2 calibration output
pub type OPA2CALOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPA3CALOUT` reader - OPAMP3 calibration output
pub type OPA3CALOUT_R = crate::BitReader;
///Field `OPA3CALOUT` writer - OPAMP3 calibration output
pub type OPA3CALOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - OPAMP1 power down
    #[inline(always)]
    pub fn opa1pd(&self) -> OPA1PD_R {
        OPA1PD_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Switch 3 for OPAMP1 enable
    #[inline(always)]
    pub fn s3sel1(&self) -> S3SEL1_R {
        S3SEL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Switch 4 for OPAMP1 enable
    #[inline(always)]
    pub fn s4sel1(&self) -> S4SEL1_R {
        S4SEL1_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Switch 5 for OPAMP1 enable
    #[inline(always)]
    pub fn s5sel1(&self) -> S5SEL1_R {
        S5SEL1_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Switch 6 for OPAMP1 enable
    #[inline(always)]
    pub fn s6sel1(&self) -> S6SEL1_R {
        S6SEL1_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - OPAMP1 offset calibration for P differential pair
    #[inline(always)]
    pub fn opa1cal_l(&self) -> OPA1CAL_L_R {
        OPA1CAL_L_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - OPAMP1 offset calibration for N differential pair
    #[inline(always)]
    pub fn opa1cal_h(&self) -> OPA1CAL_H_R {
        OPA1CAL_H_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - OPAMP1 low power mode
    #[inline(always)]
    pub fn opa1lpm(&self) -> OPA1LPM_R {
        OPA1LPM_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - OPAMP2 power down
    #[inline(always)]
    pub fn opa2pd(&self) -> OPA2PD_R {
        OPA2PD_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Switch 3 for OPAMP2 enable
    #[inline(always)]
    pub fn s3sel2(&self) -> S3SEL2_R {
        S3SEL2_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Switch 4 for OPAMP2 enable
    #[inline(always)]
    pub fn s4sel2(&self) -> S4SEL2_R {
        S4SEL2_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Switch 5 for OPAMP2 enable
    #[inline(always)]
    pub fn s5sel2(&self) -> S5SEL2_R {
        S5SEL2_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Switch 6 for OPAMP2 enable
    #[inline(always)]
    pub fn s6sel2(&self) -> S6SEL2_R {
        S6SEL2_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - OPAMP2 offset Calibration for P differential pair
    #[inline(always)]
    pub fn opa2cal_l(&self) -> OPA2CAL_L_R {
        OPA2CAL_L_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - OPAMP2 offset calibration for N differential pair
    #[inline(always)]
    pub fn opa2cal_h(&self) -> OPA2CAL_H_R {
        OPA2CAL_H_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - OPAMP2 low power mode
    #[inline(always)]
    pub fn opa2lpm(&self) -> OPA2LPM_R {
        OPA2LPM_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - OPAMP3 power down
    #[inline(always)]
    pub fn opa3pd(&self) -> OPA3PD_R {
        OPA3PD_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Switch 3 for OPAMP3 Enable
    #[inline(always)]
    pub fn s3sel3(&self) -> S3SEL3_R {
        S3SEL3_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Switch 4 for OPAMP3 enable
    #[inline(always)]
    pub fn s4sel3(&self) -> S4SEL3_R {
        S4SEL3_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Switch 5 for OPAMP3 enable
    #[inline(always)]
    pub fn s5sel3(&self) -> S5SEL3_R {
        S5SEL3_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Switch 6 for OPAMP3 enable
    #[inline(always)]
    pub fn s6sel3(&self) -> S6SEL3_R {
        S6SEL3_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - OPAMP3 offset Calibration for P differential pair
    #[inline(always)]
    pub fn opa3cal_l(&self) -> OPA3CAL_L_R {
        OPA3CAL_L_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - OPAMP3 offset calibration for N differential pair
    #[inline(always)]
    pub fn opa3cal_h(&self) -> OPA3CAL_H_R {
        OPA3CAL_H_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - OPAMP3 low power mode
    #[inline(always)]
    pub fn opa3lpm(&self) -> OPA3LPM_R {
        OPA3LPM_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Switch SanA enable for OPAMP1
    #[inline(always)]
    pub fn anawsel1(&self) -> ANAWSEL1_R {
        ANAWSEL1_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Switch SanA enable for OPAMP2
    #[inline(always)]
    pub fn anawsel2(&self) -> ANAWSEL2_R {
        ANAWSEL2_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Switch SanA enable for OPAMP3
    #[inline(always)]
    pub fn anawsel3(&self) -> ANAWSEL3_R {
        ANAWSEL3_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Switch 7 for OPAMP2 enable
    #[inline(always)]
    pub fn s7sel2(&self) -> S7SEL2_R {
        S7SEL2_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Power range selection
    #[inline(always)]
    pub fn aop_range(&self) -> AOP_RANGE_R {
        AOP_RANGE_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - OPAMP1 calibration output
    #[inline(always)]
    pub fn opa1calout(&self) -> OPA1CALOUT_R {
        OPA1CALOUT_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - OPAMP2 calibration output
    #[inline(always)]
    pub fn opa2calout(&self) -> OPA2CALOUT_R {
        OPA2CALOUT_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - OPAMP3 calibration output
    #[inline(always)]
    pub fn opa3calout(&self) -> OPA3CALOUT_R {
        OPA3CALOUT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR")
            .field("opa3calout", &self.opa3calout())
            .field("opa2calout", &self.opa2calout())
            .field("opa1calout", &self.opa1calout())
            .field("aop_range", &self.aop_range())
            .field("s7sel2", &self.s7sel2())
            .field("anawsel3", &self.anawsel3())
            .field("anawsel2", &self.anawsel2())
            .field("anawsel1", &self.anawsel1())
            .field("opa3lpm", &self.opa3lpm())
            .field("opa3cal_h", &self.opa3cal_h())
            .field("opa3cal_l", &self.opa3cal_l())
            .field("s6sel3", &self.s6sel3())
            .field("s5sel3", &self.s5sel3())
            .field("s4sel3", &self.s4sel3())
            .field("s3sel3", &self.s3sel3())
            .field("opa3pd", &self.opa3pd())
            .field("opa2lpm", &self.opa2lpm())
            .field("opa2cal_h", &self.opa2cal_h())
            .field("opa2cal_l", &self.opa2cal_l())
            .field("s6sel2", &self.s6sel2())
            .field("s5sel2", &self.s5sel2())
            .field("s4sel2", &self.s4sel2())
            .field("s3sel2", &self.s3sel2())
            .field("opa2pd", &self.opa2pd())
            .field("opa1lpm", &self.opa1lpm())
            .field("opa1cal_h", &self.opa1cal_h())
            .field("opa1cal_l", &self.opa1cal_l())
            .field("s6sel1", &self.s6sel1())
            .field("s5sel1", &self.s5sel1())
            .field("s4sel1", &self.s4sel1())
            .field("s3sel1", &self.s3sel1())
            .field("opa1pd", &self.opa1pd())
            .finish()
    }
}
impl W {
    ///Bit 0 - OPAMP1 power down
    #[inline(always)]
    pub fn opa1pd(&mut self) -> OPA1PD_W<'_, CSRrs> {
        OPA1PD_W::new(self, 0)
    }
    ///Bit 1 - Switch 3 for OPAMP1 enable
    #[inline(always)]
    pub fn s3sel1(&mut self) -> S3SEL1_W<'_, CSRrs> {
        S3SEL1_W::new(self, 1)
    }
    ///Bit 2 - Switch 4 for OPAMP1 enable
    #[inline(always)]
    pub fn s4sel1(&mut self) -> S4SEL1_W<'_, CSRrs> {
        S4SEL1_W::new(self, 2)
    }
    ///Bit 3 - Switch 5 for OPAMP1 enable
    #[inline(always)]
    pub fn s5sel1(&mut self) -> S5SEL1_W<'_, CSRrs> {
        S5SEL1_W::new(self, 3)
    }
    ///Bit 4 - Switch 6 for OPAMP1 enable
    #[inline(always)]
    pub fn s6sel1(&mut self) -> S6SEL1_W<'_, CSRrs> {
        S6SEL1_W::new(self, 4)
    }
    ///Bit 5 - OPAMP1 offset calibration for P differential pair
    #[inline(always)]
    pub fn opa1cal_l(&mut self) -> OPA1CAL_L_W<'_, CSRrs> {
        OPA1CAL_L_W::new(self, 5)
    }
    ///Bit 6 - OPAMP1 offset calibration for N differential pair
    #[inline(always)]
    pub fn opa1cal_h(&mut self) -> OPA1CAL_H_W<'_, CSRrs> {
        OPA1CAL_H_W::new(self, 6)
    }
    ///Bit 7 - OPAMP1 low power mode
    #[inline(always)]
    pub fn opa1lpm(&mut self) -> OPA1LPM_W<'_, CSRrs> {
        OPA1LPM_W::new(self, 7)
    }
    ///Bit 8 - OPAMP2 power down
    #[inline(always)]
    pub fn opa2pd(&mut self) -> OPA2PD_W<'_, CSRrs> {
        OPA2PD_W::new(self, 8)
    }
    ///Bit 9 - Switch 3 for OPAMP2 enable
    #[inline(always)]
    pub fn s3sel2(&mut self) -> S3SEL2_W<'_, CSRrs> {
        S3SEL2_W::new(self, 9)
    }
    ///Bit 10 - Switch 4 for OPAMP2 enable
    #[inline(always)]
    pub fn s4sel2(&mut self) -> S4SEL2_W<'_, CSRrs> {
        S4SEL2_W::new(self, 10)
    }
    ///Bit 11 - Switch 5 for OPAMP2 enable
    #[inline(always)]
    pub fn s5sel2(&mut self) -> S5SEL2_W<'_, CSRrs> {
        S5SEL2_W::new(self, 11)
    }
    ///Bit 12 - Switch 6 for OPAMP2 enable
    #[inline(always)]
    pub fn s6sel2(&mut self) -> S6SEL2_W<'_, CSRrs> {
        S6SEL2_W::new(self, 12)
    }
    ///Bit 13 - OPAMP2 offset Calibration for P differential pair
    #[inline(always)]
    pub fn opa2cal_l(&mut self) -> OPA2CAL_L_W<'_, CSRrs> {
        OPA2CAL_L_W::new(self, 13)
    }
    ///Bit 14 - OPAMP2 offset calibration for N differential pair
    #[inline(always)]
    pub fn opa2cal_h(&mut self) -> OPA2CAL_H_W<'_, CSRrs> {
        OPA2CAL_H_W::new(self, 14)
    }
    ///Bit 15 - OPAMP2 low power mode
    #[inline(always)]
    pub fn opa2lpm(&mut self) -> OPA2LPM_W<'_, CSRrs> {
        OPA2LPM_W::new(self, 15)
    }
    ///Bit 16 - OPAMP3 power down
    #[inline(always)]
    pub fn opa3pd(&mut self) -> OPA3PD_W<'_, CSRrs> {
        OPA3PD_W::new(self, 16)
    }
    ///Bit 17 - Switch 3 for OPAMP3 Enable
    #[inline(always)]
    pub fn s3sel3(&mut self) -> S3SEL3_W<'_, CSRrs> {
        S3SEL3_W::new(self, 17)
    }
    ///Bit 18 - Switch 4 for OPAMP3 enable
    #[inline(always)]
    pub fn s4sel3(&mut self) -> S4SEL3_W<'_, CSRrs> {
        S4SEL3_W::new(self, 18)
    }
    ///Bit 19 - Switch 5 for OPAMP3 enable
    #[inline(always)]
    pub fn s5sel3(&mut self) -> S5SEL3_W<'_, CSRrs> {
        S5SEL3_W::new(self, 19)
    }
    ///Bit 20 - Switch 6 for OPAMP3 enable
    #[inline(always)]
    pub fn s6sel3(&mut self) -> S6SEL3_W<'_, CSRrs> {
        S6SEL3_W::new(self, 20)
    }
    ///Bit 21 - OPAMP3 offset Calibration for P differential pair
    #[inline(always)]
    pub fn opa3cal_l(&mut self) -> OPA3CAL_L_W<'_, CSRrs> {
        OPA3CAL_L_W::new(self, 21)
    }
    ///Bit 22 - OPAMP3 offset calibration for N differential pair
    #[inline(always)]
    pub fn opa3cal_h(&mut self) -> OPA3CAL_H_W<'_, CSRrs> {
        OPA3CAL_H_W::new(self, 22)
    }
    ///Bit 23 - OPAMP3 low power mode
    #[inline(always)]
    pub fn opa3lpm(&mut self) -> OPA3LPM_W<'_, CSRrs> {
        OPA3LPM_W::new(self, 23)
    }
    ///Bit 24 - Switch SanA enable for OPAMP1
    #[inline(always)]
    pub fn anawsel1(&mut self) -> ANAWSEL1_W<'_, CSRrs> {
        ANAWSEL1_W::new(self, 24)
    }
    ///Bit 25 - Switch SanA enable for OPAMP2
    #[inline(always)]
    pub fn anawsel2(&mut self) -> ANAWSEL2_W<'_, CSRrs> {
        ANAWSEL2_W::new(self, 25)
    }
    ///Bit 26 - Switch SanA enable for OPAMP3
    #[inline(always)]
    pub fn anawsel3(&mut self) -> ANAWSEL3_W<'_, CSRrs> {
        ANAWSEL3_W::new(self, 26)
    }
    ///Bit 27 - Switch 7 for OPAMP2 enable
    #[inline(always)]
    pub fn s7sel2(&mut self) -> S7SEL2_W<'_, CSRrs> {
        S7SEL2_W::new(self, 27)
    }
    ///Bit 28 - Power range selection
    #[inline(always)]
    pub fn aop_range(&mut self) -> AOP_RANGE_W<'_, CSRrs> {
        AOP_RANGE_W::new(self, 28)
    }
    ///Bit 29 - OPAMP1 calibration output
    #[inline(always)]
    pub fn opa1calout(&mut self) -> OPA1CALOUT_W<'_, CSRrs> {
        OPA1CALOUT_W::new(self, 29)
    }
    ///Bit 30 - OPAMP2 calibration output
    #[inline(always)]
    pub fn opa2calout(&mut self) -> OPA2CALOUT_W<'_, CSRrs> {
        OPA2CALOUT_W::new(self, 30)
    }
    ///Bit 31 - OPAMP3 calibration output
    #[inline(always)]
    pub fn opa3calout(&mut self) -> OPA3CALOUT_W<'_, CSRrs> {
        OPA3CALOUT_W::new(self, 31)
    }
}
/**control/status register

You can [`read`](crate::Reg::read) this register and get [`csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L151.html#OPAMP:CSR)*/
pub struct CSRrs;
impl crate::RegisterSpec for CSRrs {
    type Ux = u32;
}
///`read()` method returns [`csr::R`](R) reader structure
impl crate::Readable for CSRrs {}
///`write(|w| ..)` method takes [`csr::W`](W) writer structure
impl crate::Writable for CSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR to value 0x0001_0101
impl crate::Resettable for CSRrs {
    const RESET_VALUE: u32 = 0x0001_0101;
}
