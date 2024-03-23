#[doc = "Register `SR` reader"]
pub type R = crate::R<SRrs>;
#[doc = "FIFO is ready to transfer four bytes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTT4B {
    #[doc = "0: FIFO is not ready for a four-byte transfer"]
    NotReady = 0,
    #[doc = "1: FIFO is ready for a four-byte (32-bit) transfer. In receive mode, this means that at least four valid data bytes are in the FIFO. In transmit mode, this means that there are at least four bytes free in the FIFO"]
    Ready = 1,
}
impl From<RTT4B> for bool {
    #[inline(always)]
    fn from(variant: RTT4B) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTT4B` reader - FIFO is ready to transfer four bytes"]
pub type RTT4B_R = crate::BitReader<RTT4B>;
impl RTT4B_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RTT4B {
        match self.bits {
            false => RTT4B::NotReady,
            true => RTT4B::Ready,
        }
    }
    #[doc = "FIFO is not ready for a four-byte transfer"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == RTT4B::NotReady
    }
    #[doc = "FIFO is ready for a four-byte (32-bit) transfer. In receive mode, this means that at least four valid data bytes are in the FIFO. In transmit mode, this means that there are at least four bytes free in the FIFO"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == RTT4B::Ready
    }
}
#[doc = "FIFO is ready to transfer one byte\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTT1B {
    #[doc = "0: FIFO is not ready for a 1-byte transfer"]
    NotReady = 0,
    #[doc = "1: FIFO is ready for a one byte (32-bit) transfer. In receive mode, this means that at least one valid data byte is in the FIFO. In transmit mode, this means that there is at least one byte free in the FIFO"]
    Ready = 1,
}
impl From<RTT1B> for bool {
    #[inline(always)]
    fn from(variant: RTT1B) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTT1B` reader - FIFO is ready to transfer one byte"]
pub type RTT1B_R = crate::BitReader<RTT1B>;
impl RTT1B_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RTT1B {
        match self.bits {
            false => RTT1B::NotReady,
            true => RTT1B::Ready,
        }
    }
    #[doc = "FIFO is not ready for a 1-byte transfer"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == RTT1B::NotReady
    }
    #[doc = "FIFO is ready for a one byte (32-bit) transfer. In receive mode, this means that at least one valid data byte is in the FIFO. In transmit mode, this means that there is at least one byte free in the FIFO"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == RTT1B::Ready
    }
}
impl R {
    #[doc = "Bit 2 - FIFO is ready to transfer four bytes"]
    #[inline(always)]
    pub fn rtt4b(&self) -> RTT4B_R {
        RTT4B_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FIFO is ready to transfer one byte"]
    #[inline(always)]
    pub fn rtt1b(&self) -> RTT1B_R {
        RTT1B_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "PSSI status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SRrs {}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0;
}
