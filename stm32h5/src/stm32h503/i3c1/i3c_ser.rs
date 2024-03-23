#[doc = "Register `I3C_SER` reader"]
pub type R = crate::R<I3C_SERrs>;
#[doc = "Field `CODERR` reader - protocol error code/type controller detected an illegally formatted CCC controller detected that transmitted data on the bus is different from expected controller detected a not acknowledged broadcast address (7’hE) controller detected the new controller did not drive bus after controller-role hand-off target detected an invalid broadcast address 7’hE+W target detected a parity error on a CCC code via a parity check (vs T bit) target detected a parity error on a write data via a parity check (vs T bit) target detected a parity error on the assigned address during dynamic address arbitration via a parity check (vs PAR bit) target detected a 7’hE+R missing after Sr during dynamic address arbitration target detected an illegally formatted CCC target detected that transmitted data on the bus is different from expected others: reserved"]
pub type CODERR_R = crate::FieldReader;
#[doc = "Field `PERR` reader - protocol error"]
pub type PERR_R = crate::BitReader;
#[doc = "Field `STALL` reader - SCL stall error (when the I3C is acting as target)"]
pub type STALL_R = crate::BitReader;
#[doc = "Field `DOVR` reader - RX-FIFO overrun or TX-FIFO underrun i) a TX-FIFO underrun: TX-FIFO is empty and a write data byte has to be transmitted ii) a RX-FIFO overrun: RX-FIFO is full and a new data byte is received"]
pub type DOVR_R = crate::BitReader;
#[doc = "Field `COVR` reader - C-FIFO underrun or S-FIFO overrun (when the I3C is acting as controller) i) a C-FIFO underrun: control FIFO is empty and a restart has to be emitted ii) a S-FIFO overrun: S-FIFO is full and a new message ends"]
pub type COVR_R = crate::BitReader;
#[doc = "Field `ANACK` reader - address not acknowledged (when the I3C is configured as controller) i) a legacy I2C read/write transfer ii) a direct CCC write transfer iii) the second trial of a direct CCC read transfer iv) a private read/write transfer"]
pub type ANACK_R = crate::BitReader;
#[doc = "Field `DNACK` reader - data not acknowledged (when the I3C is acting as controller) i) a legacy I2C write transfer ii) the second trial when sending dynamic address during ENTDAA procedure"]
pub type DNACK_R = crate::BitReader;
#[doc = "Field `DERR` reader - data error (when the I3C is acting as controller)"]
pub type DERR_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - protocol error code/type controller detected an illegally formatted CCC controller detected that transmitted data on the bus is different from expected controller detected a not acknowledged broadcast address (7’hE) controller detected the new controller did not drive bus after controller-role hand-off target detected an invalid broadcast address 7’hE+W target detected a parity error on a CCC code via a parity check (vs T bit) target detected a parity error on a write data via a parity check (vs T bit) target detected a parity error on the assigned address during dynamic address arbitration via a parity check (vs PAR bit) target detected a 7’hE+R missing after Sr during dynamic address arbitration target detected an illegally formatted CCC target detected that transmitted data on the bus is different from expected others: reserved"]
    #[inline(always)]
    pub fn coderr(&self) -> CODERR_R {
        CODERR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - protocol error"]
    #[inline(always)]
    pub fn perr(&self) -> PERR_R {
        PERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SCL stall error (when the I3C is acting as target)"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RX-FIFO overrun or TX-FIFO underrun i) a TX-FIFO underrun: TX-FIFO is empty and a write data byte has to be transmitted ii) a RX-FIFO overrun: RX-FIFO is full and a new data byte is received"]
    #[inline(always)]
    pub fn dovr(&self) -> DOVR_R {
        DOVR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - C-FIFO underrun or S-FIFO overrun (when the I3C is acting as controller) i) a C-FIFO underrun: control FIFO is empty and a restart has to be emitted ii) a S-FIFO overrun: S-FIFO is full and a new message ends"]
    #[inline(always)]
    pub fn covr(&self) -> COVR_R {
        COVR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - address not acknowledged (when the I3C is configured as controller) i) a legacy I2C read/write transfer ii) a direct CCC write transfer iii) the second trial of a direct CCC read transfer iv) a private read/write transfer"]
    #[inline(always)]
    pub fn anack(&self) -> ANACK_R {
        ANACK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - data not acknowledged (when the I3C is acting as controller) i) a legacy I2C write transfer ii) the second trial when sending dynamic address during ENTDAA procedure"]
    #[inline(always)]
    pub fn dnack(&self) -> DNACK_R {
        DNACK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - data error (when the I3C is acting as controller)"]
    #[inline(always)]
    pub fn derr(&self) -> DERR_R {
        DERR_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "I3C status error register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i3c_ser::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3C_SERrs;
impl crate::RegisterSpec for I3C_SERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3c_ser::R`](R) reader structure"]
impl crate::Readable for I3C_SERrs {}
#[doc = "`reset()` method sets I3C_SER to value 0"]
impl crate::Resettable for I3C_SERrs {
    const RESET_VALUE: u32 = 0;
}
